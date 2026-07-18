#!/bin/sh
# PHI-marker guard — a backstop to the structural "no-PII-field" guarantee, NOT a
# substitute (the field that does not exist is what carries the weight). Synthetic
# fixtures must stay synthetic.
#   - email, or SSN pattern (NNN-NN-NNNN)  -> flagged
#   - date of birth: an ISO date ONLY when a birth-context word shares its line
#     (a bare ISO date is a doc/spec date, not a DOB — flagging every date is noise).
# A deliberate synthetic value carries a same-line SYNTHETIC-OK marker.
mode="$1"; fail=0
EMAIL='[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,}'
SSN='[0-9]{3}-[0-9]{2}-[0-9]{4}'
ISO='(19|20)[0-9]{2}-(0[1-9]|1[0-2])-(0[1-9]|[12][0-9]|3[01])'
BIRTH='([Dd]\.?[Oo]\.?[Bb]|[Bb]irth|[Bb]orn)'
MARK='SYNTHETIC-OK'
scan() {
  grep -nE "$EMAIL|$SSN" "$1" 2>/dev/null | grep -v "$MARK" | sed "s|^|  PHI shape in $1: |"
  grep -nE "$ISO" "$1" 2>/dev/null | grep -E "$BIRTH" | grep -v "$MARK" | sed "s|^|  DOB shape in $1: |"
}
case "$mode" in
  diff) files=$(git diff --cached --name-only --diff-filter=ACM) ;;
  tree) files=$(git ls-files) ;;
  *) echo "usage: phi-scan.sh diff|tree"; exit 2 ;;
esac
for f in $files; do
  case "$f" in *.png|*.jpg|*.woff2|*.svg|LICENSE*) continue ;; esac
  hits=$(scan "$f"); [ -n "$hits" ] && { echo "$hits"; fail=1; }
done
[ "$fail" -eq 0 ] && echo "phi-scan: clean" || echo "phi-scan: FAIL — PHI shape found (mark a deliberate synthetic value with SYNTHETIC-OK on the same line)"
exit $fail
