use tokio::net::{TcpListener, TcpStream};
use tokio::sync::broadcast;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    let (tx, _) = broadcast::channel::<(u16,String)>(100);

    println!("Server running on 127.0.0.1:8080");

    loop {
        let (socket, addr) = listener.accept().await?;
        println!("Client connected: {}", addr);

        let tx = tx.clone();
        let rx = tx.subscribe();

        tokio::spawn(async move {
            handle_client(socket,addr, tx, rx).await;
        });
    }
}

async fn handle_client(
    socket: TcpStream,
    addr: core::net::SocketAddr,
    tx: broadcast::Sender<(u16,String)>,
    mut rx: broadcast::Receiver<(u16,String)>,
) {
    let (reader, mut writer) = socket.into_split();
    let mut reader = BufReader::new(reader);
    let mut line = String::new();

    loop {
        tokio::select! {

            

            // Lire depuis le client
            result = reader.read_line(&mut line) => {
                match result {
                    Ok(0) => break, // déconnexion
                    Ok(_) => {
                        let msg = line.clone();
                        tx.send((addr.port(), msg)).ok();
                        line.clear();
                    }
                    Err(_) => break,
                }
            }

            // Recevoir depuis les autres clients
            result = rx.recv() => {
                match result {
                    Ok(msg) => {
                        if msg.0 != addr.port() {
                            if writer.write_all(msg.1.as_bytes()).await.is_err() {
                                break;
                            }
                        }
                        
                    }
                    Err(e) => match e {
                        broadcast::error::RecvError::Lagged(n) => {
                            eprint!("Warning: {} messages perdus", n);
                        }
                        broadcast::error::RecvError::Closed => break,
                    }
                }
            }
        }
    }

    println!("Le client s'est déconnecté.");
}