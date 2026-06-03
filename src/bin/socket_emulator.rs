use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpListener;
use tokio::sync::Mutex;

#[derive(Debug)]
struct SocketState {
    on: bool,
    power: f64,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <bind_addr> <power>", args[0]);
        eprintln!("Example: {} 127.0.0.1:1234 150.0", args[0]);
        std::process::exit(1);
    }
    let bind_addr = &args[1];
    let power: f64 = args[2].parse().expect("Invalid power");

    let listener = TcpListener::bind(bind_addr).await?;
    let state = Arc::new(Mutex::new(SocketState { on: false, power }));

    println!("Socket emulator listening on {}", bind_addr);

    loop {
        let (stream, _peer) = match listener.accept().await {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Accept error: {}", e);
                continue;
            }
        };

        let st = state.clone();
        tokio::spawn(async move {
            let (r, mut w) = stream.into_split();
            let mut reader = BufReader::new(r).lines();

            while let Ok(Some(line)) = reader.next_line().await {
                let cmd = line.trim().to_uppercase();
                let resp = match cmd.as_str() {
                    "ON" => {
                        let mut s = st.lock().await;
                        s.on = true;
                        "OK".to_string()
                    }
                    "OFF" => {
                        let mut s = st.lock().await;
                        s.on = false;
                        "OK".to_string()
                    }
                    "STATE" => {
                        let s = st.lock().await;
                        if s.on {
                            "ON".to_string()
                        } else {
                            "OFF".to_string()
                        }
                    }
                    "POWER" => {
                        let s = st.lock().await;
                        if s.on {
                            format!("{}", s.power)
                        } else {
                            "0".to_string()
                        }
                    }
                    _ => "ERROR".to_string(),
                };

                if let Err(e) = w.write_all(resp.as_bytes()).await {
                    eprintln!("write error: {}", e);
                    break;
                }
                if let Err(e) = w.write_all(b"\n").await {
                    eprintln!("write error: {}", e);
                    break;
                }
            }
        });
    }
}
