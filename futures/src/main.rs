extern crate futures;
//use futures::future::empty;

mod input;
mod timeout;
use futures::future;
use std::io;
use timeout::Timeout;


fn main() {
    match read_name(){
        Err(_) => println!("Hello, whatever your name is."),
        Ok(name)=> println!("Hello {:?}", name),
    }
}

fn read_name() -> io::Result<String> {
    use std::time::Duration;

    let result = futures::future::empty()
        .select(Timeout::new(Duration::from_secs(10), || {
            io::Error::new(io::ErrorKind::Other, "timeout elapsed".to_string())
        }))
        .wait();

        match result {
            Ok((name, _)) => Ok(name),
            Err((e, _)) => Err(e),
        }
}
