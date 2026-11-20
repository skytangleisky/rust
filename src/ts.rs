
use std::io::Read;
use std::net::TcpListener;
use std::time::Instant;

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:9000")?;

    println!("TCP Server listening on 9000");

    for stream in listener.incoming() {
        let mut stream = stream?;

        std::thread::spawn(move || {
            let mut buf = vec![0u8; 256 * 1024];
            let mut total: u64 = 0;
            let mut last = Instant::now();

            loop {
                match stream.read(&mut buf) {
                    Ok(0) => break,
                    Ok(n) => {
                        total += n as u64;

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
                    Err(e) => {
                        eprintln!("Read error: {}", e);
                        break;
                    }
                }
            }
        });
    }

    Ok(())
}

// rust-script src/ts.rs
// cargo run --bin ts