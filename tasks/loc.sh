#!/bin/bash
# Count lines of code in only .rs, .b, and .sh files, output total to lines.txt
total=$(find . -type f \( -name "*.rs" -o -name "*.b" -o -name "*.sh" \) -not -path "./target/*" | xargs wc -l | awk '/total/ {print $1}')

# Generate shields.io badge and put it in lines.md
badge_url="https://img.shields.io/badge/lines-$total-blue?style=flat-square"

readme="README.md"
tmpfile="README.tmp"

# Remove any existing badge line at the top, preserving all other lines and spacing
awk 'NR==1 && /^!\[Lines of code\]\(https:\/\/img\.shields\.io\/badge\/lines-[0-9]+-blue\?style=flat-square\)/{next} {print}' "$readme" > "$tmpfile"

# Insert the badge at the very top, followed by the rest of the file
{ echo "![Lines of code]($badge_url)"; cat "$tmpfile"; } > "$readme"

rm "$tmpfile"