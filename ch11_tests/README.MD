cargo new adder --lib

cargo test

cargo test --help

cargo test -- --test-threads=1

cargo test -- --show-output

cargo test wildcard
module name is part of test name

cargo test --test filename

cargo test -- --ignored
cargo test -- --include-ignored

===

The three sections of output include the unit tests, the integration test, and the doc tests. 
Note that if any test in a section fails, the following sections will not be run.

===

Files in subdirectories of the tests directory don’t get compiled as separate crates or have 
sections in the test output.


===

Only library crates expose functions that other crates can use; binary crates are meant to 
be run on their own.