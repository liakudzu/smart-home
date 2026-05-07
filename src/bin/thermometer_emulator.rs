use rand::RngExt;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::net::UdpSocket;
use std::thread;
use std::time::Duration;

fn main() {
    let config = match File::open("thermometer.conf") {
        Ok(f) => f,
        Err(_) => {
            eprintln!("Config file 'thermometer.conf' not found.");
            eprintln!("Create it with two lines: <target_addr> and <period_ms>");
            eprintln!("Example: 127.0.0.1:8888\n1000");
            std::process::exit(1);
        }
    };
    let reader = BufReader::new(config);
    let lines: Vec<String> = reader.lines().map_while(Result::ok).collect();
    if lines.len() < 2 {
        eprintln!("Config file must contain target_addr and period_ms");
        std::process::exit(1);
    }
    let target_addr = &lines[0];
    let period_ms: u64 = lines[1].parse().expect("Invalid period");
    let socket = UdpSocket::bind("0.0.0.0:0").expect("Failed to bind UDP socket");
    let mut rng = rand::rng();

    println!(
        "Thermometer emulator started. Sending to {} every {} ms",
        target_addr, period_ms
    );
    loop {
        let temp = rng.random_range(-30.0..30.0);
        let msg = format!("{:.2}", temp);
        if let Err(e) = socket.send_to(msg.as_bytes(), target_addr) {
            eprintln!("Failed to send: {}", e);
        }
        thread::sleep(Duration::from_millis(period_ms));
    }
}
