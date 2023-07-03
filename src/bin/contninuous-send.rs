use clap::Parser;
use serialport::ClearBuffer;
use std::time::Duration;

#[derive(Debug, Parser)]
pub struct AppConfig {
    /// The baud rate to send data at.
    #[arg(long, default_value_t = 115200)]
    baud: u32,
    /// Timeout for writing a single pattern in ms.
    #[arg(long)]
    timeout_ms: Option<u64>,
    /// Clear buffers before initially writing data.
    #[arg(long, default_value_t = true)]
    clear: bool,
    /// Flush output after writing the pattern once.
    #[arg(long, default_value_t = false)]
    flush: bool,
    /// The serial device to send data with.
    device: String,
}

fn main() {
    let config = AppConfig::parse();
    let mut port = serialport::new(config.device, config.baud).open().unwrap();

    println!("port: {:?}", port);
    println!("timeout: {:?}", port.timeout());

    if let Some(timeout_ms) = config.timeout_ms {
        port.set_timeout(Duration::from_millis(timeout_ms)).unwrap();
    }

    if config.clear {
        port.clear(ClearBuffer::All).unwrap();
    }

    let pattern =
        b" 0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~";

    loop {
        port.write_all(pattern).unwrap();
        if config.flush {
            port.flush().unwrap();
        }
    }
}
