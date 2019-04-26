use std::io;
use std::thread;
use std::time::Duration;
use std::io::prelude::*;
use multisock::{SocketAddr,Stream};
use std::env;

fn main() -> battery::Result<()> {
    let manager = battery::Manager::new()?;
    let mut battery = match manager.batteries()?.next() {
        Some(Ok(battery)) => battery,
        Some(Err(e)) => {
            eprintln!("Unable to access battery information");
            return Err(e);
        }
        None => {
            eprintln!("Unable to find any batteries");
            return Err(io::Error::from(io::ErrorKind::NotFound).into());
        }
    };

    // Config
    let addr = env::var("BATT_POST_ADDR")
        .unwrap_or(String::from("127.0.0.1:34254"));

    // Set up socket and buffer
    let mut stream = Stream::connect(&addr.parse().unwrap())?;
    let mut buf = String::new();

    loop {
        buf = battery.state_of_charge().value.to_string();
        stream.write_all(&buf.into_bytes())?;
        stream.write_all(b"\n")?;
        thread::sleep(Duration::from_secs(1));
        manager.refresh(&mut battery)?;
    }
}
