use std::collections::HashMap;
use std::error::Error;
use std::io;

fn main() -> Result<(), Box<dyn Error>> {
    let mut map = HashMap::<String, String>::new();

    loop {
        let mut line = String::new();
        io::stdin().read_line(&mut line)?;

        // TODO
    }
}
