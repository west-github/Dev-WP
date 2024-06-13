use anyhow::Result;




use std::{
    thread,
    time::{Duration, Instant},
};

fn work() -> Result<i32> {
    let mut file = std::fs::OpenOptions::new()
        .append(true)
        .create(true)
        .read(true)
        .open("index.txt")?;

    // let mut file = std::io::BufReader::new(file);

    let mut buffer = [0; 20];
    let (tx, rv) = std::sync::mpsc::channel::<String>();

    let closure = move || loop {
        thread::sleep(Duration::from_secs(1));
        use std::io::Read;
        let b_read = match file.read(&mut buffer) {
            Ok(b) if b == 0 => break,
            Ok(b) => b,
            Err(e) => {
                println!("Error happened while reading to buffer: {}", e);
                break;
            }
        };

        if let Err(e) = tx.send(clean_buf_to_string(&buffer[0..b_read])) {
            println!("Error happened while sending to channel: {}", e);
            break;
        }
    };

    std::thread::spawn(closure);

    let mut count = 0;
    while let Ok(_message) = rv.recv() {
        count += 1;
    }

    Ok(count)
}

fn main() -> Result<()> {
    println!("{:>20}", "DEV - RUNNING");
    for _ in 0..10 {
        let now = std::time::Instant::now();
        let count = work()?;
        println!("Count: {} - {:?}", count, Instant::now().duration_since(now));
    }

    println!("From the main thread ");

    Ok(())
}

fn clean_buf_to_string(buf: &[u8]) -> String {
    String::from_utf8_lossy(buf)
        .to_string()
        .replace("\n", "")
        .replace("\r", "")
}
