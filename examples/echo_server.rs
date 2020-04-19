//! An echo server with Tokio
//!
//! This server will create a TCP listener, accept connections in a loop, and
//! write back everything that's read off of each TCP connection.
//!
//! Because the Tokio runtime uses a thread pool, each TCP connection is
//! processed concurrently with all other TCP connections across multiple
//! threads.
//!
//! To see this server in action, you can run this in one terminal:
//!
//!     cargo run --example echo
//!
//! and in another terminal you can run:
//!
//!     cargo run --example connect 127.0.0.1:6142
//!
//! Each line you type in to the `connect` terminal should be echo'd back to
//! you!

#![warn(rust_2018_idioms)]

use std::env;
use tokio::net::TcpListener;
use tokio::stream::StreamExt;

#[tokio::main]
async fn main() {
    // Allow passing an address to listen on as the first argument of this
    // program, but otherwise we'll just set up our TCP listener on
    // 127.0.0.1:6142 for connections.
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:6142".to_string());

    // Next up we create a TCP listener which will listen for incoming
    // connections. This TCP listener is bound to the address we determined
    // above and must be associated with an event loop.
    println!("Listening on: {}", addr);
    let mut listener = TcpListener::bind(addr).await.unwrap();

    // Here we convert the `TcpListener` to a stream of incoming connections
    // with the `incoming` method.
    let server = async move {
        let mut incoming = listener.incoming();
        while let Some(socket_res) = incoming.next().await {
            match socket_res {
                Ok(mut socket) => {
                    let peer_addr = socket.peer_addr().unwrap();
                    println!("Accepted connection from {}", peer_addr);

                    // Spawn the future that echos the data and returns how
                    // many bytes were copied as a concurrent task.
                    tokio::spawn(async move {
                        // Split up the reading and writing parts of the
                        // socket.
                        let (mut reader, mut writer) = socket.split();
                        match tokio::io::copy(&mut reader, &mut writer).await {
                            Ok(amt) => {
                                println!("{}: wrote a total of {} bytes", peer_addr, amt);
                            }
                            Err(err) => {
                                eprintln!("{}: IO error {:?}", peer_addr, err);
                            }
                        }
                    });
                }
                Err(err) => println!("accept err {:?}", err),
            }
        }
    };

    // Start the server and block this async fn until `server` spins down.
    server.await;
}
