use std::net::UdpSocket;
use std::env;

fn main() -> std::io::Result<()> {
	let args: Vec<String> = env::args().collect();
	let port: i32;
	if args.len() == 2 {
		let result: Result<i32, _> = args[1].parse();
		port = match result {
			Ok(n) => n,
			Err(_) => {
				println!("Invalid port provided, using 1235");
				1235
			}
		}
	}
	else {
		port = 1235;
	}
	println!("Listening on port {}", port.to_string());
	let socket = UdpSocket::bind(format!("127.0.0.1:1235"))?;
	let mut buf = [0u8; 1024];
	loop {
		let (amt, _src) = socket.recv_from(&mut buf)?;
		println!("{}", String::from_utf8_lossy(&buf[..amt]));
	}
}