CSSACT - Cascading StyleSheets At Compile-Time
----------------------------------------------

A CSS minimizer and validator implemented as a syntax extension for the Rust programming language.

The goal of this project is to be able to compress and minimize CSS at compile-time and create good and detailed error messages when the syntax is wrong, but at the same time don't get in the way of writing the intended CSS.

Currently the method of doing all this is accomplished by using a part of the Servo project, specifically the stylesheet parser. This may be an impractical and inefficient way of solving it, and there are still many issues left to be solved and also stuff that are not implemented correctly yet.
