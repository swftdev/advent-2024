use std::io;

mod days;


fn main() -> io::Result<()> {
    let _ = days::day1::day1();
    Ok(())
}
