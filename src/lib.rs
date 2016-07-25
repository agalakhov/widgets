mod os;
mod widgets;

#[cfg(windows)]
pub use os::winapi::*;

pub fn run() -> Result<(), Error> {
    while try!(run_once()) {
    }
    return Ok(());
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
    }

}
