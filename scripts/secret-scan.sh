#!/bin/sh
# Secret scan — mirrors beehive-nature's shape and its documented exemptions.
#   flagged: secret-bearing filenames, 48+ hex runs, PEM private-key blocks
#   exempt : a line carrying a same-line PUBLIC-CONSTANT or TESTNET-ONLY marker.
#            (Public content digests and public chain data are key-SHAPED but not key
#            material. The marker must be on the SAME line as the hex, so an unmarked
#            run still fails — the exemption is per-line, never per-file.)
mode="$1"; fail=0
NAME_RE='\.(seed|key|pem|secret)$|(^|/)secrets/|(^|/)\.env(\.|$)'
HEX_RE='[0-9a-fA-F]{48,}'
PEM_RE='BEGIN .*PRIVATE KE[Y]'
MARK='PUBLIC-CONSTANT|TESTNET-ONLY'
case "$mode" in
  diff) files=$(git diff --cached --name-only --diff-filter=ACM) ;;
  tree) files=$(git ls-files) ;;
  *) echo "usage: secret-scan.sh diff|tree"; exit 2 ;;
esac
# Fail closed. Enumeration is via git, so outside a repo (or if git errors) $files is
# empty, the loop below never runs, and this script would print "clean" having inspected
# NOTHING — a guard that passes without looking. In tree mode a repo always has tracked
# files, so empty means the enumeration failed, not that the tree is empty. An empty
# staged set in diff mode is legitimate (nothing to check), so only tree mode refuses.
if [ "$mode" = tree ] && [ -z "$files" ]; then
  echo "  secret-scan: REFUSING — git ls-files returned nothing." >&2
  echo "  Not a git repository, or git failed. A scan of zero files is not a pass." >&2
  exit 2
fi
for f in $files; do
  echo "$f" | grep -qE "$NAME_RE" && { echo "  secret-shaped filename: $f"; fail=1; }
  case "$f" in *.woff2|*.png|*.jpg|*.svg) continue ;; esac
  [ -f "$f" ] || continue
  hex=$(grep -nE "$HEX_RE" "$f" 2>/dev/null | grep -vE "$MARK")
  [ -n "$hex" ] && { echo "$hex" | sed "s|^|  unmarked 48+ hex in $f: |"; fail=1; }
  pem=$(grep -nE "$PEM_RE" "$f" 2>/dev/null)
  [ -n "$pem" ] && { echo "  PEM private key in: $f"; fail=1; }
done
[ "$fail" -eq 0 ] && echo "secret-scan: clean"
exit $fail
