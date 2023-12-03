#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(rust_2018_idioms)]

use futures_util::{future, pin_mut, StreamExt};
use std::error::Error;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = url::Url::parse("ws://ltnm.learncppthroughprojects.com:80/echo").unwrap();

    let (stdin_tx, stdin_rx) = futures_channel::mpsc::unbounded();

    let stdin_tx1 = stdin_tx.clone();
    let stdin_tx2 = stdin_tx.clone();

    tokio::spawn(async move {
        let _ = stdin_tx1.unbounded_send(Message::Text("Hello, world!".into()));
    });

    tokio::spawn(async move {
        let _ = stdin_tx2.unbounded_send(Message::Text("General Kenobi!".into()));
    });

    let manager = tokio::spawn(async {
        let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");
        println!("WebSocket handshake has been successfully completed");

        let (write, read) = ws_stream.split();

        let stdin_to_ws = stdin_rx.map(Ok).forward(write);
        let ws_to_stdout = {
            read.for_each(|message| async move {
                println!("{:?}", message);
            })
        };

        pin_mut!(stdin_to_ws, ws_to_stdout);
        future::select(stdin_to_ws, ws_to_stdout).await;
    });

    let _ = tokio::join!(manager);

    Ok(())
}
