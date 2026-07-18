#!/bin/sh
# Secret scan (mirrors beehive-nature): secret-bearing filenames, 48+ hex runs, PEM.
mode="$1"; fail=0
NAME_RE='\.(seed|key|pem|secret)$|(^|/)secrets/|(^|/)\.env(\.|$)'
HEX_RE='[0-9a-fA-F]{48,}'
PEM_RE='BEGIN .*PRIVATE KE[Y]'
case "$mode" in
  diff) files=$(git diff --cached --name-only --diff-filter=ACM) ;;
  tree) files=$(git ls-files) ;;
  *) echo "usage: secret-scan.sh diff|tree"; exit 2 ;;
esac
for f in $files; do
  echo "$f" | grep -qE "$NAME_RE" && { echo "  secret-shaped filename: $f"; fail=1; }
  case "$f" in *.woff2|*.png|*.jpg) continue ;; esac
  grep -qE "$HEX_RE" "$f" 2>/dev/null && { echo "  48+ hex run in: $f"; fail=1; }
  grep -qE "$PEM_RE" "$f" 2>/dev/null && { echo "  PEM private key in: $f"; fail=1; }
done
[ "$fail" -eq 0 ] && echo "secret-scan: clean"
exit $fail
