/// Prints the collected `errors` as a prominent block at the end of a run.
pub fn print_errors(errors: &[String]) {
    println!();
    if errors.is_empty() {
        println!("=== no errors ===");
    } else {
        println!("=== {} error(s) ===", errors.len());
        for error in errors {
            println!("  {error}");
        }
    }
}
