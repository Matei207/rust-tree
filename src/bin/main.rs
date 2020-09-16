use std::{env, io};
fn main() -> io::Result<()> {
    let args = env::args();
    rust_tree::run(args)?;

    Ok(())
}

/*
    * '?' is shorthand for an entire match statement, if it's an error
    it returns from the function, if it is an ok it can unwraps
*/
