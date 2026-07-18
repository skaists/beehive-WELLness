#!/bin/sh
# PHI-marker guard — a backstop to the structural "no-PII-field" guarantee, NOT a
# substitute (the field that does not exist is what carries the weight). Synthetic
# fixtures must stay synthetic.
#   - email, or SSN pattern (NNN-NN-NNNN)  -> flagged
#   - date of birth: an ISO date ONLY when a birth-context word shares its line
#     (a bare ISO date is a doc/spec date, not a DOB — flagging every date is noise).
# Two same-line markers exempt a line, and they mean DIFFERENT things. Keeping them
# distinct matters: an exemption that misstates why it exists is a false claim sitting
# inside a guard, which is the defect class this repository keeps having to repair.
#   SYNTHETIC-OK   - the value is FABRICATED. Test fixtures, example records.
#   PUBLIC-CONTACT - the value is REAL and its publication is a deliberate, ruled
#                    decision. A maintainer's disclosure address is not PHI: it is
#                    nobody's health information, it belongs to the person publishing
#                    it, and a security policy without a route is not a policy.
# Do not reach for SYNTHETIC-OK to silence a real value - it would make the marker lie.
mode="$1"; fail=0
EMAIL='[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,}'
SSN='[0-9]{3}-[0-9]{2}-[0-9]{4}'
ISO='(19|20)[0-9]{2}-(0[1-9]|1[0-2])-(0[1-9]|[12][0-9]|3[01])'
BIRTH='([Dd]\.?[Oo]\.?[Bb]|[Bb]irth|[Bb]orn)'
MARK='SYNTHETIC-OK|PUBLIC-CONTACT'
scan() {
  grep -nE "$EMAIL|$SSN" "$1" 2>/dev/null | grep -vE "$MARK" | sed "s|^|  PHI shape in $1: |"
  grep -nE "$ISO" "$1" 2>/dev/null | grep -E "$BIRTH" | grep -vE "$MARK" | sed "s|^|  DOB shape in $1: |"
}
case "$mode" in
  diff) files=$(git diff --cached --name-only --diff-filter=ACM) ;;
  tree) files=$(git ls-files) ;;
  *) echo "usage: phi-scan.sh diff|tree"; exit 2 ;;
esac
# Fail closed — see secret-scan.sh for the full reasoning. In tree mode an empty file
# list means enumeration failed, not that there is nothing to check; a scan of zero
# files must not read as a pass.
if [ "$mode" = tree ] && [ -z "$files" ]; then
  echo "  phi-scan: REFUSING — git ls-files returned nothing." >&2
  echo "  Not a git repository, or git failed. A scan of zero files is not a pass." >&2
  exit 2
fi
for f in $files; do
  case "$f" in *.png|*.jpg|*.woff2|*.svg|LICENSE*) continue ;; esac
  hits=$(scan "$f"); [ -n "$hits" ] && { echo "$hits"; fail=1; }
done
[ "$fail" -eq 0 ] && echo "phi-scan: clean" || echo "phi-scan: FAIL — PHI shape found. Same-line marker required: SYNTHETIC-OK if the value is fabricated, PUBLIC-CONTACT if it is real and deliberately published. Pick the one that is true."
exit $fail
