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
