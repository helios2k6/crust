/*************************************************************************
 * This file was generated by CRUST by an automated semantics preserving
 * translation from C/C++ to Rust
 * CRUST isn't perfect and may require manual editing
 * Check warnings and errors and refer to the official Rust Documentation
 ************************************************************************/
fn main() {
    /*Avoid using mutable variables unless it is necessary to do so
     */
    let mut argv: Vec<_> = std::env::args().collect();
    let mut argc = argv.len();
    /** Crust tries to identify return statement and replace with rust equivalent
     * shorthand notation. If error found in this line, Please replace shorthand notation
     * with return statement
     **/
    std::process::exit(123);
}
fn hello() -> i32 {
    /** Crust tries to identify return statement and replace with rust equivalent
     * shorthand notation. If error found in this line, Please replace shorthand notation
     * with return statement
     **/
    1
}
