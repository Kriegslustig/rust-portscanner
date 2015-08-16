use std::net::TcpStream;
use std::thread;

fn main() {
  let host = "127.0.0.1";
  let from_port : u16 = 1;
  let to_port : u16 = 1000;
  println!("Scanning {} to {} on {}", from_port, to_port, host);
  println!("Open tcp ports on {}", host);
  let result = (from_port..to_port)
    .map(|port| {
      thread::spawn(move || {
        (port, scan_port(host, port))
      })
    })
    .map(|handle| {
      handle.join().unwrap()
    })
    .collect::<Vec<_>>()
    .into_iter()
    .filter(|scan| scan.1);
  for scan in result {
    println!("{}", scan.0);
  }
}

fn scan_port(host: &str, port: u16) -> bool {
  let connection = TcpStream::connect((host, port));
  connection.is_ok()
}