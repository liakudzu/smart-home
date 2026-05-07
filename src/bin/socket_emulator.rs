use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

struct SocketState {
    on: bool,
    power: f64,
}

fn handle_client(mut stream: TcpStream, state: Arc<Mutex<SocketState>>) {
    let reader_stream = match stream.try_clone() {
        Ok(reader_stream) => reader_stream,
        Err(_) => return,
    };
    let mut reader = BufReader::new(reader_stream);

    loop {
        let mut cmd = String::new();
        match reader.read_line(&mut cmd) {
            Ok(0) => break,
            Ok(_) => {
                let cmd = cmd.trim().to_uppercase();
                let response = match cmd.as_str() {
                    "ON" => {
                        let mut s = state.lock().unwrap();
                        s.on = true;
                        "OK".to_string()
                    }
                    "OFF" => {
                        let mut s = state.lock().unwrap();
                        s.on = false;
                        "OK".to_string()
                    }
                    "STATE" => {
                        let s = state.lock().unwrap();
                        if s.on {
                            "ON".to_string()
                        } else {
                            "OFF".to_string()
                        }
                    }
                    "POWER" => {
                        let s = state.lock().unwrap();
                        if s.on {
                            format!("{}", s.power)
                        } else {
                            "0".to_string()
                        }
                    }
                    _ => "ERROR".to_string(),
                };
                let _ = stream.write_all(response.as_bytes());
                let _ = stream.write_all(b"\n");
                let _ = stream.flush();
            }
            Err(_) => break,
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <bind_addr> <power>", args[0]);
        eprintln!("Example: {} 127.0.0.1:1234 150.0", args[0]);
        std::process::exit(1);
    }
    let bind_addr = &args[1];
    let power: f64 = args[2].parse().expect("Invalid power");
    let listener = TcpListener::bind(bind_addr).expect("Failed to bind");
    let state = Arc::new(Mutex::new(SocketState { on: false, power }));

    println!("Socket emulator listening on {}", bind_addr);
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let state_clone = Arc::clone(&state);
                thread::spawn(move || {
                    handle_client(stream, state_clone);
                });
            }
            Err(e) => eprintln!("Connection failed: {}", e),
        }
    }
}
