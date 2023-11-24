use futures_util::stream::StreamExt;
use futures_util::SinkExt;
use http::Uri;
use tokio::{
    io::{AsyncBufReadExt, BufReader},
    select,
};
use tokio_websockets::{ClientBuilder, Message};

#[tokio::main]
async fn main() -> Result<(), tokio_websockets::Error> {
    let (mut ws_stream, _) = ClientBuilder::from_uri(Uri::from_static("ws://127.0.0.1:2000"))
        .connect()
        .await?;

    let stdin = tokio::io::stdin();
    let mut stdin = BufReader::new(stdin).lines();

    loop {
        select! {
            response = ws_stream.next() => {
                match response {
                    Some(result) => {
                        if let Some(text) = result?.as_text() {
                            println!("{}", text);
                        }
                    },
                    None => {
                        println!("connection to server lost");
                        return Ok(());
                    },
                }
            },
            Ok(result) = stdin.next_line() => {
                match result {
                    Some(text) => ws_stream.send(Message::text(text)).await?,
                    None => {
                        // end of stdin stream
                        return Ok(());
                    },
                }
            },
        };
    }
}
