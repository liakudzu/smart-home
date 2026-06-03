use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tokio::net::UdpSocket;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
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

    let socket = UdpSocket::bind("0.0.0.0:0").await?;

    let mut seed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos() as u64;
    println!(
        "Thermometer emulator started. Sending to {} every {} ms",
        target_addr, period_ms
    );

    loop {
        // simple LCG for emulator randomness
        seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
        let frac = seed as f64 / u64::MAX as f64;
        let temp = -30.0 + frac * 60.0;
        let msg = format!("{:.2}", temp);
        if let Err(e) = socket.send_to(msg.as_bytes(), target_addr).await {
            eprintln!("Failed to send: {}", e);
        }
        tokio::time::sleep(Duration::from_millis(period_ms)).await;
    }
}
