use tokio::net::TcpStream;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let stream = TcpStream::connect("127.0.0.1:8080").await?;

    let (reader, mut writer) = stream.into_split();

    let mut reader = BufReader::new(reader);

    let mut stdin = BufReader::new(tokio::io::stdin());
    let mut input = String::new();
    let mut server_msg = String::new();

    print!("Pseudo : ");
    let _ = stdin.read_line(&mut input).await?;
    let username = input.replace("\n", "");
    input.clear();
    loop {
        
        tokio::select! {

            // écrire au serveur
            _ = stdin.read_line(&mut input) => {
                let msg = format!("[{username}] : {input}");
                writer.write_all(msg.as_bytes()).await?;
                input.clear();
            }

            // lire du serveur
            _ = reader.read_line(&mut server_msg) => {
                print!("> {}", server_msg);
                server_msg.clear();
            }
        }
    }
}