#!/bin/sh
# verify-l1.sh — reproduce the Level 1 grant yourself. Order E, Tier R.
#
#   Do not take our word for it. Here is the command.
#   If your result differs from ours, ours is wrong.
#
# WHAT THIS CHECKS
#   A1a  zero external references in the graded artifacts
#   A1b  the bytes a reader RECEIVES OVER THE WIRE hash to the recorded digests
#   D1   the delivery limitation is disclosed (presence, not adequacy — see below)
#
# WHAT THIS DOES NOT CHECK, and will not pretend to:
#   - whether the digests were recorded BEFORE publication or back-filled to match.
#     That rests on the git history, which this script does not audit.
#   - whether the operator serves the same bytes tomorrow. This is point-in-time.
#   - whether the artifacts render with egress actually blocked. Zero external
#     references implies it by construction; nobody watched a browser do it.
#   - whether the disclosure is ADEQUATE. This reports that text is present and
#     prints it. You judge it. A script must not pretend to exercise judgement it
#     cannot exercise.
#
# REQUIRES: sh, grep, sed, curl, and sha256sum (or shasum). Nothing else. No token,
# no login, no membership, no network writes, no mutation of this tree.
# Portability is a security property here: a verifier that only runs in one
# environment is only checkable by whoever has that environment.
#
# USAGE
#   ./verify/verify-l1.sh                 # verify the served surfaces
#   ./verify/verify-l1.sh --base <url>    # verify against a different origin
#   ./verify/verify-l1.sh selftest        # prove this script FAILS when it should
#
# EXIT CODES
#   0  every check passed
#   1  a check failed
#   2  REFUSED — could not form a verdict (empty set, missing tool, missing input).
#      Refusal is not a pass. A verdict over an empty set is a missing test.

set -u

DEFAULT_BASE="https://skaists.github.io/beehive-WELLness"
TABLE="surfaces/README.md"
# D1 is checked PER ARTIFACT, not directory-level. A reader arriving by direct link never
# sees a directory README, and D1 as ratified says the model is disclosed *in the artifact*.
# The wording is byte-identical in all eight, which is what lets this be an exact check
# rather than a fuzzy one — and makes drift or alteration trivially visible.
DISCLOSURE_TEXT="Served by one operator who can log every reader and withdraw this page at will. Not censorship-resistant, not unobserved."

BASE="$DEFAULT_BASE"
MODE="verify"
ROOT="."

while [ $# -gt 0 ]; do
    case "$1" in
        selftest)  MODE="selftest" ;;
        --base)    shift; [ $# -gt 0 ] || { echo "REFUSE: --base needs a value" >&2; exit 2; }; BASE="$1" ;;
        --root)    shift; [ $# -gt 0 ] || { echo "REFUSE: --root needs a value" >&2; exit 2; }; ROOT="$1" ;;
        -h|--help) sed -n '2,40p' "$0"; exit 0 ;;
        *)         echo "REFUSE: unknown argument '$1'" >&2; exit 2 ;;
    esac
    shift
done

# ── portability shims ──────────────────────────────────────────────────────────
if command -v sha256sum >/dev/null 2>&1; then
    sha256_of() { sha256sum "$1" | cut -d' ' -f1; }
elif command -v shasum >/dev/null 2>&1; then
    sha256_of() { shasum -a 256 "$1" | cut -d' ' -f1; }
else
    echo "REFUSE: no sha256sum or shasum on PATH. Cannot hash; refusing rather than skipping." >&2
    exit 2
fi
command -v curl >/dev/null 2>&1 || { echo "REFUSE: curl not on PATH." >&2; exit 2; }

# Fetch one artifact into $2. A local path base reads from disk (used by selftest);
# an http(s) base fetches over the wire with caching defeated.
fetch_one() {
    _name="$1"; _out="$2"
    case "$BASE" in
        http://*|https://*)
            curl -sS -L --max-time 30 -H 'Cache-Control: no-cache' -H 'Pragma: no-cache' \
                 -o "$_out" -w '%{http_code} %{url_effective}' \
                 "$BASE/surfaces/$_name" 2>/dev/null
            ;;
        *)
            if [ -f "$BASE/$_name" ]; then cp "$BASE/$_name" "$_out"; printf '200 %s' "$BASE/$_name"
            else printf '404 %s' "$BASE/$_name"; fi
            ;;
    esac
}

# ── the run ────────────────────────────────────────────────────────────────────
run_verify() {
    _root="$1"
    _table="$_root/$TABLE"
    [ -f "$_table" ] || { echo "REFUSE: no digest table at $_table" >&2; return 2; }

    # Enumerate the graded artifacts from the recorded digest table.
    _names=$(grep -oE '^\| `[A-Za-z0-9_]+\.html`' "$_table" | sed 's/^| `//; s/`$//')
    _digests=$(grep -E '^\| `[A-Za-z0-9_]+\.html`' "$_table" | grep -oE '[0-9a-f]{64}')

    # LAW 1a — non-emptiness precedes any verdict. A scan of zero files is not a pass.
    _n=$(printf '%s\n' "$_names" | grep -c . 2>/dev/null || echo 0)
    _nd=$(printf '%s\n' "$_digests" | grep -c . 2>/dev/null || echo 0)
    if [ "$_n" -eq 0 ]; then
        echo "REFUSE: enumerated 0 artifacts from the digest table." >&2
        echo "        A verdict over an empty set is a missing test, not a pass." >&2
        return 2
    fi
    if [ "$_n" -ne "$_nd" ]; then
        echo "REFUSE: $_n filenames but $_nd digests — table is malformed." >&2
        return 2
    fi

    echo "artifacts_enumerated=$_n"
    _fail=0
    _extfail=0
    _digfail=0

    # ── A1a · zero external references ─────────────────────────────────────────
    _scanned=0
    for _f in $_names; do
        _p="$_root/surfaces/$_f"
        [ -f "$_p" ] || { echo "  MISSING local artifact: $_f"; _fail=1; continue; }
        _scanned=$((_scanned + 1))
        # "External reference" means something the BROWSER FETCHES — not any string
        # that happens to contain a URL. A bare `https://…` printed as text in a code
        # block is inert: nothing requests it. An earlier version of this check was a
        # blunt `https?://` grep, which is over-broad in a way that matters here: it
        # would forbid this tree from ever printing a URL as text, including in the
        # lessons about checking sources that are its whole subject.
        #
        # So the check tests fetch POSITIONS: a remote URL in a fetching attribute, in
        # CSS @import or url(), or handed to a runtime fetch API. The selftest proves
        # each of those forms is still caught — refining a check without re-proving it
        # bites is how a gate becomes a decoration.
        _hits=$(grep -nEi \
'(src|href|action|poster|srcset|formaction|cite|manifest|codebase|archive|longdesc|background|profile)[[:space:]]*=[[:space:]]*.?[[:space:]]*(https?:)?//|@import|url\([[:space:]]*.?[[:space:]]*(https?:)?//|integrity[[:space:]]*=|crossorigin|(fetch|XMLHttpRequest|WebSocket|EventSource|importScripts|sendBeacon)[[:space:]]*\(|\.src[[:space:]]*=' \
            "$_p" 2>/dev/null)
        if [ -n "$_hits" ]; then
            echo "  EXTERNAL REFERENCE in $_f:"
            printf '%s\n' "$_hits" | sed 's/^/    /'
            _extfail=1; _fail=1
        fi
    done
    if [ "$_scanned" -eq 0 ]; then
        echo "REFUSE: scanned 0 files for external references." >&2
        return 2
    fi
    [ "$_extfail" -eq 0 ] && echo "A1a_external_references=0 (over $_scanned files)" \
                          || echo "A1a_external_references=FOUND"

    # ── A1b · served bytes hash to the recorded digests ─────────────────────────
    _tmp="${TMPDIR:-/tmp}/verify-l1.$$"
    mkdir -p "$_tmp" || { echo "REFUSE: cannot create temp dir" >&2; return 2; }
    _fetched=0; _matched=0
    _i=0
    for _f in $_names; do
        _i=$((_i + 1))
        _rec=$(printf '%s\n' "$_digests" | sed -n "${_i}p")
        _meta=$(fetch_one "$_f" "$_tmp/$_f")
        _code=$(printf '%s' "$_meta" | cut -d' ' -f1)
        _eff=$(printf '%s' "$_meta" | cut -d' ' -f2-)
        if [ "$_code" != "200" ]; then
            echo "  FETCH $_code for $_f  ($_eff)"; _digfail=1; _fail=1; continue
        fi
        # A 200 from the wrong host is not a pass.
        case "$_eff" in
            "$BASE"*) : ;;
            *) echo "  WRONG ORIGIN for $_f: expected $BASE, got $_eff"; _digfail=1; _fail=1; continue ;;
        esac
        _fetched=$((_fetched + 1))
        _got=$(sha256_of "$_tmp/$_f")
        if [ "$_got" = "$_rec" ]; then
            _matched=$((_matched + 1))
        else
            echo "  DIGEST MISMATCH $_f"
            echo "    served   $_got"
            echo "    recorded $_rec"
            _digfail=1; _fail=1
        fi
    done
    rm -rf "$_tmp"

    if [ "$_fetched" -eq 0 ]; then
        echo "REFUSE: fetched 0 artifacts; nothing was compared." >&2
        return 2
    fi
    echo "A1b_fetched=$_fetched served_matches_recorded=$_matched"

    # ── D1 · disclosure present IN EACH ARTIFACT (presence, NOT adequacy) ───────
    # Exact-string match, because the wording is byte-identical across all eight.
    # That makes both absence AND alteration fail: a reworded disclosure is drift,
    # and drift in a limitation is how a limitation quietly stops being stated.
    # This reports presence and leaves adequacy to the reader.
    _dis=0
    for _f in $_names; do
        _p="$_root/surfaces/$_f"
        [ -f "$_p" ] || continue
        if grep -qF "$DISCLOSURE_TEXT" "$_p" 2>/dev/null; then
            _dis=$((_dis + 1))
        else
            echo "  D1 DISCLOSURE MISSING OR ALTERED in $_f"
            _fail=1
        fi
    done
    echo "D1_disclosure_in_artifacts=$_dis/$_n"

    echo "RESULT=$([ "$_fail" -eq 0 ] && echo PASS || echo FAIL)"
    return "$_fail"
}

# ── selftest · Law 2 applied to this tool ──────────────────────────────────────
# A verifier that has only ever been observed passing is a decoration. This project
# has shipped two gates that only ever passed: scanners that reported "clean" over
# zero files, and a grep -v basic-regex bug that silently disabled an exemption and
# was caught only because a POSITIVE control failed.
run_selftest() {
    _src="$1"
    _work="${TMPDIR:-/tmp}/verify-l1-selftest.$$"
    rm -rf "$_work"; mkdir -p "$_work" || { echo "REFUSE: cannot create work dir" >&2; exit 2; }
    _pass=0; _failed=0

    _case() { # name expected_code
        _cn="$1"; _want="$2"
        ( BASE="$_work/$_cn/served"; run_verify "$_work/$_cn/tree" ) >"$_work/$_cn.out" 2>&1
        _got=$?
        if [ "$_got" -eq "$_want" ]; then
            printf '  ok    %-28s exit=%s (expected %s)\n' "$_cn" "$_got" "$_want"
            _pass=$((_pass + 1))
        else
            printf '  FAIL  %-28s exit=%s (expected %s)\n' "$_cn" "$_got" "$_want"
            sed 's/^/          /' "$_work/$_cn.out"
            _failed=$((_failed + 1))
        fi
    }

    _mk() { # build a fixture: name
        _n="$1"
        mkdir -p "$_work/$_n/tree/surfaces" "$_work/$_n/served"
        cp "$_src/surfaces/README.md" "$_work/$_n/tree/surfaces/"
        for _f in "$_src"/surfaces/*.html; do
            cp "$_f" "$_work/$_n/tree/surfaces/"
            cp "$_f" "$_work/$_n/served/"
        done
    }

    echo "selftest: fixtures asserting this script fails when it should — and passes when it should"

    _mk intact;                                                        _case intact 0

    # Every fetchable form gets its own fixture. The check was refined to stop flagging
    # inert text; each of these proves the refinement did not open a hole.
    _inject() { # name, replacement-html
        _mk "$1"
        sed -i.bak "s|</head>|$2</head>|" \
            "$_work/$1/tree/surfaces/module3_portion_reality.html" 2>/dev/null
        rm -f "$_work/$1/tree/surfaces/"*.bak
        _case "$1" 1
    }
    _inject external_script  '<script src=\"https://cdn.example.com/x.js\"></script>'
    _inject external_link    '<link rel=\"stylesheet\" href=\"https://cdn.example.com/x.css\">'
    _inject css_import       '<style>@import url(\"https://cdn.example.com/x.css\");</style>'
    _inject css_url          '<style>body{background:url(https://cdn.example.com/bg.png)}</style>'
    _inject runtime_fetch    '<script>fetch(\"https://api.example.com/beacon\")</script>'
    _inject protocol_relative '<script src=\"//cdn.example.com/x.js\"></script>'

    # And the control that motivated the refinement: a URL as INERT TEXT must NOT fail.
    _mk inert_text_url
    sed -i.bak 's|</body>|<p>Clone it: git clone https://github.com/skaists/beehive-WELLness</p></body>|' \
        "$_work/inert_text_url/tree/surfaces/module3_portion_reality.html" 2>/dev/null
    rm -f "$_work/inert_text_url/tree/surfaces/"*.bak
    # the served copy must match the mutated tree copy, else the digest check fires instead
    cp "$_work/inert_text_url/tree/surfaces/module3_portion_reality.html" \
       "$_work/inert_text_url/served/module3_portion_reality.html"
    # recompute that one digest in the fixture's table so only the ref-check is under test
    _newd=$(sha256_of "$_work/inert_text_url/tree/surfaces/module3_portion_reality.html")
    _oldd=$(grep -E '^\| `module3_portion_reality\.html`' "$_work/inert_text_url/tree/surfaces/README.md" | grep -oE '[0-9a-f]{64}')
    sed -i.bak "s/$_oldd/$_newd/" "$_work/inert_text_url/tree/surfaces/README.md" 2>/dev/null
    rm -f "$_work/inert_text_url/tree/surfaces/"*.bak
    _case inert_text_url 0

    _mk flipped_byte
    # size- and reference-preserving mutation: digest must catch it alone
    _t="$_work/flipped_byte/served/bqueenbee_analytics.html"
    sed -i.bak '1s/^<!doctype html>/<!DOCTYPE html>/' "$_t" 2>/dev/null
    rm -f "$_work/flipped_byte/served/"*.bak
    _case flipped_byte 1

    # ── D1 per-artifact fixtures (order D1-IA-3) ───────────────────────────────
    # THE ONE THAT CARRIES THE ARGUMENT: the disclosure removed from a SINGLE
    # artifact while the directory README stays intact. Under the old
    # directory-level check this PASSED. Under the per-artifact check it FAILS.
    # Watching that flip is the demonstration that the refinement did something —
    # a refined check nobody watched change behaviour is a refactor, not a fix.
    _mk d1_missing_from_one
    _dt="$_work/d1_missing_from_one/tree/surfaces/module2_fats_label_wont_show.html"
    sed -i.bak "s/ · Served by one operator who can log every reader and withdraw this page at will\. Not censorship-resistant, not unobserved\.//" "$_dt" 2>/dev/null
    rm -f "$_work/d1_missing_from_one/tree/surfaces/"*.bak
    cp "$_dt" "$_work/d1_missing_from_one/served/module2_fats_label_wont_show.html"
    _rd=$(sha256_of "$_dt")
    _od=$(grep -E '^\| `module2_fats_label_wont_show\.html`' "$_work/d1_missing_from_one/tree/surfaces/README.md" | grep -oE '[0-9a-f]{64}')
    sed -i.bak "s/$_od/$_rd/" "$_work/d1_missing_from_one/tree/surfaces/README.md" 2>/dev/null
    rm -f "$_work/d1_missing_from_one/tree/surfaces/"*.bak
    _case d1_missing_from_one 1

    # disclosure ALTERED rather than absent — identical wording is the property,
    # so drift must bite as hard as deletion
    _mk d1_altered
    _at="$_work/d1_altered/tree/surfaces/module3_portion_reality.html"
    sed -i.bak "s/Not censorship-resistant, not unobserved\./Not censorship resistant, not unobserved./" "$_at" 2>/dev/null
    rm -f "$_work/d1_altered/tree/surfaces/"*.bak
    cp "$_at" "$_work/d1_altered/served/module3_portion_reality.html"
    _rd=$(sha256_of "$_at")
    _od=$(grep -E '^\| `module3_portion_reality\.html`' "$_work/d1_altered/tree/surfaces/README.md" | grep -oE '[0-9a-f]{64}')
    sed -i.bak "s/$_od/$_rd/" "$_work/d1_altered/tree/surfaces/README.md" 2>/dev/null
    rm -f "$_work/d1_altered/tree/surfaces/"*.bak
    _case d1_altered 1

    # present in the README only, absent from every artifact
    _mk d1_readme_only
    for _g in "$_work/d1_readme_only/tree/surfaces"/*.html; do
        sed -i.bak "s/ · Served by one operator who can log every reader and withdraw this page at will\. Not censorship-resistant, not unobserved\.//" "$_g" 2>/dev/null
        cp "$_g" "$_work/d1_readme_only/served/$(basename "$_g")"
        _rd=$(sha256_of "$_g"); _bn=$(basename "$_g")
        _od=$(grep -E "^\| \`$_bn\`" "$_work/d1_readme_only/tree/surfaces/README.md" | grep -oE '[0-9a-f]{64}')
        sed -i.bak2 "s/$_od/$_rd/" "$_work/d1_readme_only/tree/surfaces/README.md" 2>/dev/null
    done
    rm -f "$_work/d1_readme_only/tree/surfaces/"*.bak "$_work/d1_readme_only/tree/surfaces/"*.bak2
    _case d1_readme_only 1

    _mk empty_set
    # a digest table with no rows: MUST refuse, never pass
    printf '# surfaces\n\nno table here\n' > "$_work/empty_set/tree/surfaces/README.md"
    _case empty_set 2

    _mk wrong_origin
    # the served copy is a different artifact: a 200 from the wrong bytes is not a pass
    cp "$_src/surfaces/module1_hemp_hearts_label_reader.html" \
       "$_work/wrong_origin/served/module3_portion_reality.html"
    _case wrong_origin 1

    rm -rf "$_work"
    echo "selftest: $_pass passed, $_failed failed"
    [ "$_failed" -eq 0 ] || return 1
    return 0
}

# ── main ───────────────────────────────────────────────────────────────────────
if [ "$MODE" = selftest ]; then
    run_selftest "$ROOT"; exit $?
fi

echo "=== verify-l1 · reproducing the Level 1 grant ==="
echo "base=$BASE"
echo "commit=$(git -C "$ROOT" rev-parse HEAD 2>/dev/null || echo '(not a git checkout)')"
echo "---"
run_verify "$ROOT"
_rc=$?
echo "---"
if [ "$_rc" -eq 0 ]; then
    echo "L1 REPRODUCED. Compare this block against grants/ — character for character."
elif [ "$_rc" -eq 2 ]; then
    echo "REFUSED — no verdict formed. This is not a pass."
else
    echo "L1 NOT REPRODUCED. If you believe your result is correct, ours is wrong."
    echo "Tell us: see SECURITY.md."
fi
exit "$_rc"
