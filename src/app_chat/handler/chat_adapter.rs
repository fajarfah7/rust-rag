use axum::body::Bytes;
use futures::Stream;
use tokio::time::{sleep, Duration};

pub fn string_to_stream(
    s: String,
) -> impl Stream<Item = Result<Bytes, std::io::Error>> {
    let (tx, rx) = tokio::sync::mpsc::channel(8);

    tokio::spawn(async move {
        for chunk in s.as_bytes().chunks(10) {
            tx.send(Ok(Bytes::copy_from_slice(chunk))).await.ok();

            sleep(Duration::from_millis(50)).await;
        }
    });

    tokio_stream::wrappers::ReceiverStream::new(rx)
}
