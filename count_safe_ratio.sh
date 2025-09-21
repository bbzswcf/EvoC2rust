#!/bin/bash
count_safe_ratio() {
    a=$(tree-grepper -q rust '((function_item (function_modifiers) @_m)@f (#match? @_m "unsafe"))' | grep ':f:' | wc -l)
    b=$(tree-grepper -q rust '((function_item)@f)' | grep :f: | wc -l)

    awk -v a="$a" -v b="$b" 'BEGIN {
        if (b == 0) {
            ratio = 0;
        } else {
            ratio = 100 - a * 100 / b;
        }
        printf "// safe API ratio is: %.2f%%\n", ratio
    }'

    a=$(tree-grepper -q rust '((unsafe_block) @b)' | sed -e 's/^.*:b://' | wc -c)
    c=$(tree-grepper -q rust '((function_item (function_modifiers) @_m)@f (#match? @_m "unsafe"))' | sed -e 's/^.*:f://' | wc -c)
    b=$(tree-grepper -q rust '((function_item)@f)' | sed -e 's/^.*:f://' | wc -c)

    awk -v a="$a" -v b="$b" -v c="$c" 'BEGIN {
        if (b == 0) {
            ratio = 0;
        } else {
            ratio = 100 - (a + c) * 100 / b;
        }
        printf "// Safe code ratio is: %.2f%%\n", ratio
    }'
}

count_safe_ratio