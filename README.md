# Expression Identity Verifier in Rust

A program that takes expressions from the following domains: {Strings, Sets, Boolean, Algebra} and verifies whether their identity holds. Implements the Shunting Yard algorithm in Rust.

Example Input:

```xml
let xml = r#"
        <strings>
            2 * 3 + 1 = 2 + 2 + 2 + 1 
            <algebra>
                 2 * 3 + 1 = (1 + 1) * 2 + 2 + 1 = 7
                 <sets>
                       {1, 2} + ({1, 2, 3} * {2, 3}) = ({1, 2} + {1, 2, 3}) * {2, 3}
                 </sets>
                 1 + 2 * 2 + 1 = 2 + 2 + 2 * 1;         
            </algebra>
            <boolean>
                 (1 + 0) * 1 + 1 = 0 * 1 + 1
            </boolean>
            1 * (2 + 1) + 1 = 1 + 1 + 1 
        </strings>
    "#;
```

This project may be updated later to read in the xml from a file, but for now it loads it as a let binding r#""#; string.

Running the command `cargo run` will produce this output:

```javascript
================================================
    Statement:  2 * 3 + 1 = 2 + 2 + 2 + 1,
    Domain:     Strings,
    Resolved To:    7 == 7,
    Valid:      true
================================================
    Statement:  2 * 3 + 1 = (1 + 1) * 2 + 2 + 1 = 7,
    Domain:     Algebra,
    Resolved To:    7 == 7 == 7,
    Valid:      true
================================================
    Statement:  {1, 2} + ({1, 2, 3} * {2, 3}) = ({1, 2} + {1, 2, 3}) * {2, 3},
    Domain:     Sets,
    Resolved To:    {3, 2, 1} == {2, 3},
    Valid:      false
================================================
    Statement:  1 + 2 * 2 + 1 = 2 + 2 + 2 * 1;,
    Domain:     Algebra,
    Resolved To:    6 == 6,
    Valid:      true
================================================
    Statement:  (1 + 0) * 1 + 1 = 0 * 1 + 1,
    Domain:     Boolean,
    Resolved To:    1 == 1,
    Valid:      true
================================================
    Statement:  1 * (2 + 1) + 1 = 1 + 1 + 1,
    Domain:     Strings,
    Resolved To:    4 == 3,
    Valid:      false
================================================
```

This project implements the following context free grammar:

```
Grammar for the `binding control`:
directory_scope → < identifier > block_objects </ identifier >
block_objects → various_express scope_change
various_exprs → various_express ; new_expr | new_expr
scope_change → directory_scope various_exprs | directory_scope | ԑ
identifier → letter identifier | letter
letter →  a|b|c|d|e|f|g|i|j|k|l|m|n|o|p|q|r|s|t|uv|w|x|y|z

Grammar for `sets`:
new_expr → set_expr | new_expr  =  set_expr
set_expr  → set_expr set_union set_term | set_term
set_term   → set_term set_intersection set_factor | set_factor
set_factor → ( set_expr ) | { element_list }
element_list → element_list , element_list | single_element
single_element → letter | digit
digit → 0|1|2|3|4|5|6|7|9
letter → a|b|c|d|e|f|g|i|j|k|l|m|n|o|p|q|r|s|t|uv|w|x|y|z
set_union  → +
set_intersection → *

Grammar for `algebra`:
new_expr → algebra_expr | new_expr = algebra_expr
algebra_expr  → algebra_expr plus algebra_term | algebra_expr minus algebra_term 
algebra_expr → algebra_term | minus algebra_term
algebra_term   → algebra_term times algebra_factor | algebra_factor
algebra_factor → algebra_raised power algebra_factor | algebra_raised
algebra_raised → ( algebra_expr ) | algebra_identifier
algebra_identifier → 0|1|2|3|4|5|6|7|9
plus → + 
minus →  -
times → *
power → ^

Grammar for `boolean`:
new_expr → boolean_expr | new_expr = Boolean_expr
boolean_expr  → boolean_expr or boolean_term | boolean_term
boolean_term → boolean_term and boolean_factor boolean_factor
boolean_factor → ( boolean_expr ) | boolean_identifier
boolean_identifier → 0|1
or → +
and → *
```
Grammar for `strings`:
new_expr → string_expr | new_expr = string_expr
string_expr  → string_expr concatenation string_term | string_term
string_term → string_term power string_factor | string_factor
string_factor → ( string_expr ) | digit
digit → 0|1|2|3|4|5|6|7|8|9
concatenation → +
power → *

