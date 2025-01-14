use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::str;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    let mut stream = TcpStream::connect("127.0.0.1:8080").await?;
    println!("created stream");

    let mut recv_buf = [0; 1024];
    loop {
        let mut send_buf = String::new();
        std::io::stdin().read_line(&mut send_buf).unwrap();

        let result = stream.write(send_buf.as_bytes()).await;
        println!("send size: {}", send_buf.len());
        println!("wrote to stream; success={:?}", result.is_ok());

        let n = match stream.read(&mut recv_buf).await {
            Ok(n) if n == 0 => break,
            Ok(n) => n,
            Err(e) => {
                eprintln!("failed to read from server; err = {:?}", e);
                break;
            }
        };

        let recv_str = str::from_utf8(&recv_buf[4..n])
            .unwrap_or_else(|_| "invalid UTF-8 sequence returned");

        println!("received size: {}", n);
        println!("result: {}", recv_str);
    }

    Ok(())
}