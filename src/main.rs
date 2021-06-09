use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    io::stdin().read_line(buf)

    handle.read_to_string(&mut buffer)?;

    println!("{}", buffer);

    Ok(())
}
