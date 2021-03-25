use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt");
    match f {
        Ok(file) => file,
        Err(err) => println!("Problem opening file {:?}", err),
    };

    let f2 = File::open("hello.txt");

    /// this fails compiling
    /// let f2 = match f2 {
    ///     Ok(file) => file,
    ///     Err(err) => match err.kind() {
    ///         ErrorKind::NotFound => match File::create("hello.txt") {
    ///             Ok(fc) => fc,
    ///             Err(e) => panic!("Problem creating the file: {:?}", e),
    ///         },
    ///         other_error => {
    ///             panic!("Problem opening the file: {:?}", other_error)
    ///         }
    ///     },
    /// };


}
