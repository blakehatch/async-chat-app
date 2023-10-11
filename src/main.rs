use tokio::{ 
    io::{AsyncWriteExt, AsyncBufReadExt, BufReader}, 
    net::TcpListener,
    sync::broadcast };


#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("localhost:8080").await.unwrap();

    let (tx, mut rx) = broadcast::channel::<String>(10);

    let tx = tx.clone();

    loop {
        let (mut socket, _addr) = listener.accept().await.unwrap();

        tokio::spawn(async move {
            let (reader, mut writer) = socket.split();

            let mut reader = BufReader::new(reader);
            let mut line = String::new();

            loop {
                
                let bytes_read = reader.read_line(&mut line).await.unwrap();
                if bytes_read == 0 {
                    break;
                }

                tx.send(line.clone()).unwrap();

                let msg = rx.recv().await.unwrap();

                writer.write_all(line.as_bytes()).await.unwrap();
                line.clear();
            }
        });
    }

}
