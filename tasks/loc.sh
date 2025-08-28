#!/bin/bash
# Count lines of code in only .rs, .b, and .sh files, output total to lines.txt
total=$(find . -type f \( -name "*.rs" -o -name "*.b" -o -name "*.sh" \) -not -path "./target/*" | xargs wc -l | awk '/total/ {print $1}')

# Generate shields.io badge and put it in lines.md
badge_url="https://img.shields.io/badge/lines-$total-blue?style=flat-square"

# Update README.md: remove all badge lines at the top, then insert the new badge with a single newline after
readme="README.md"
tmpfile="README.tmp"

awk 'NR==1{title=$0; next} 
     /^!\[Lines of code\]\(https:\/\/img\.shields\.io\/badge\/lines-[0-9]+-blue\?style=flat-square\)/{next} 
     NF || !/^$/ {print}' "$readme" > "$tmpfile"

# Insert the badge and title at the top, with only one newline after the badge
{ echo "![Lines of code]($badge_url)"; echo "$title"; cat "$tmpfile"; } > "$readme"

rm "$tmpfile"