#!/usr/bin/env python3
"""
Fix Pattern.from_list() calls to use new API: Pattern.pattern(value, Pattern.from_values(list))
"""
import re
import sys

def fix_from_list(content):
    """Replace Pattern.from_list(value, list) with Pattern.pattern(value, Pattern.from_values(list))"""
    
    # Pattern 1: Simple single-line calls
    # Pattern.from_list("value", [items])  ->  Pattern.pattern("value", Pattern.from_values([items]))
    pattern1 = r'Pattern\.from_list\(([^,]+),\s*(\[[^\]]*\])\)'
    content = re.sub(pattern1, r'Pattern.pattern(\1, Pattern.from_values(\2))', content)
    
    # Pattern 2: Calls with variable as second argument
    # Pattern.from_list("value", items)  ->  Pattern.pattern("value", Pattern.from_values(items))
    pattern2 = r'Pattern\.from_list\(([^,]+),\s*([a-zA-Z_][a-zA-Z0-9_]*)\)'
    content = re.sub(pattern2, r'Pattern.pattern(\1, Pattern.from_values(\2))', content)
    
    # Pattern 3: Calls with list() or range()
    # Pattern.from_list("value", list(...))  ->  Pattern.pattern("value", Pattern.from_values(list(...)))
    pattern3 = r'Pattern\.from_list\(([^,]+),\s*((?:list|range)\([^)]*\))\)'
    content = re.sub(pattern3, r'Pattern.pattern(\1, Pattern.from_values(\2))', content)
    
    # Pattern 4: Single argument (no value)
    # Pattern.from_list(items)  ->  Pattern.pattern("root", Pattern.from_values(items))
    pattern4 = r'Pattern\.from_list\(([a-zA-Z_][a-zA-Z0-9_]*)\)'
    content = re.sub(pattern4, r'Pattern.pattern("root", Pattern.from_values(\1))', content)
    
    return content

if __name__ == "__main__":
    if len(sys.argv) < 2:
        print("Usage: python fix_from_list.py <file>")
        sys.exit(1)
    
    filename = sys.argv[1]
    with open(filename, 'r') as f:
        content = f.read()
    
    fixed = fix_from_list(content)
    
    with open(filename, 'w') as f:
        f.write(fixed)
    
    print(f"Fixed {filename}")
