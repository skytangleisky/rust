
use std::io::Write;
use std::net::TcpStream;
use std::time::Instant;

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:9000")?;

    stream.set_nodelay(true)?;

    let buf = vec![0u8; 256 * 1024];
    let mut total: u64 = 0;
    let mut last = Instant::now();

    loop {
        stream.write_all(&buf)?;
        total += buf.len() as u64;

        if last.elapsed().as_secs() >= 1 {
            let speed = total as f64 / last.elapsed().as_secs_f64();

            println!(
                "Throughput: {:.2} MB/s | {:.2} Mbps",
                speed / 1024.0 / 1024.0,
                speed * 8.0 / 1_000_000.0
            );

            total = 0;
            last = Instant::now();
        }
    }
}

// rust-script src/tc.rs
// cargo run --bin tc