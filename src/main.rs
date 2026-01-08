
use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use std::{io::ErrorKind};

#[tokio::main]
async fn main () -> Result<(), Box<dyn std::error::Error>> {

	let listener = TcpListener::bind("127.0.0.1:8080").await?;

	loop {
		let (mut socket, _) = listener.accept().await?;

		println!("New connection : {}", socket.peer_addr()?);

		tokio::spawn(async move {
			let mut buf = [0; 1024];

			loop {
			 let bytes_read = match socket.read(&mut buf).await {
					Ok(0) => return,
					Ok(bytes_read) => bytes_read,
					Err(e) if e.kind() == ErrorKind::ConnectionAborted => {
						println!("Disconnected");
						return;
					},
					Err(e) => {
						println!("error: {e}");
						return;
					},
				};

				println!("Read : {:?}", &buf[0..bytes_read]);
			}
		});
	}
}
