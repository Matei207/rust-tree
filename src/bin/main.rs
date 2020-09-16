use std::io;
//TODO: add cfg to parse cli options
fn main() -> io::Result<()> {
    rust_tree::run()?;

    Ok(())
}

/*
    * '?' is shorthand for an entire match statement, if it's an error
    it returns from the function, if it is an ok it can unwraps
*/
