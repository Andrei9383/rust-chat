use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::tcp::OwnedWriteHalf;
use tokio::net::TcpListener;
use tokio::sync::Mutex;

use std::collections::HashMap;
use std::{io::ErrorKind, net::SocketAddr, sync::Arc};

type Db = Arc<Mutex<HashMap<SocketAddr, (String, OwnedWriteHalf)>>>;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    let connections: Db = Arc::new(Mutex::new(HashMap::new()));

    loop {
        let (socket, addr) = listener.accept().await?;

        let db = connections.clone();

        tokio::spawn(async move {
            let (mut reader, writer) = socket.into_split();

            let mut buf = [0; 1024];

            {
                let mut map = db.lock().await;

                if !map.contains_key(&addr) {
                    map.insert(addr, ("Anonymous".to_string(), writer));
                }

                let value = map.get_mut(&addr);
                let (name, writer) = value.unwrap();

                println!("User {} has joined!", name);
                let welcome_message = format!("Hello, {}!", name);

                let _ = writer.write_all(welcome_message.as_bytes()).await;
            }

            loop {
                let bytes_read = match reader.read(&mut buf).await {
                    Ok(0) => return,
                    Ok(bytes_read) => bytes_read,
                    Err(e) if e.kind() == ErrorKind::ConnectionAborted => {
                        println!("Disconnected");
                        return;
                    }
                    Err(e) => {
                        println!("error: {e}");
                        return;
                    }
                };

                if buf[..bytes_read].starts_with(b"/broadcast") {
                    let whole_text = String::from_utf8_lossy(&buf[0..bytes_read]);

                    let message = whole_text.split_once(" ").unwrap_or((" ", " ")).1;

                    let mut map = db.lock().await;

                    let sender_name = map.get(&addr).unwrap().0.clone();

                    for (client_addr, (_name, client_writer)) in map.iter_mut() {
                        if addr != *client_addr {
                            let format = format!("{}: {}", sender_name, message);

                            let _ = client_writer.write_all(format.as_bytes()).await;
                        }
                    }
                    continue;
                }

                if buf[..bytes_read].starts_with(b"/nickname") {
                    let text = String::from_utf8_lossy(&buf[..bytes_read]);

                    if text.contains(" ") {
                        println!("Error when parsing: invalid nickname");
                    }

                    let name = match text.trim().split_once(" ") {
                        Some((_, name)) => name,
                        None => "Anonymous",
                    };

                    let mut map = db.lock().await;

                    let mut old_name = "Anonymous".to_string();

                    map.entry(addr).and_modify(|old_value| {
                        old_name = old_value.0.to_owned();
                        old_value.0 = name.to_string()
                    });

                    println!("{} has updated their name to {}!", old_name, name);

                    continue;
                }

                if !buf[..bytes_read].starts_with(b"/broadcast") {
                    continue;
                }

                let text = String::from_utf8_lossy(&buf[0..bytes_read]);

                println!("Detected command : {}", text);
            }
        });
    }
}
