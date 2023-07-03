use clap::Parser;
use serialport::ClearBuffer;
use std::io::Write;
use std::{io::ErrorKind, time::Duration};

#[derive(Debug, Parser)]
pub struct AppConfig {
    /// The baud rate to read with.
    #[arg(long, default_value_t = 115200)]
    baud: u32,
    /// Timeout for reading a chunk of data ms.
    #[arg(long)]
    timeout_ms: Option<u64>,
    /// Clear buffers before initially reading data.
    #[arg(long, default_value_t = true)]
    clear: bool,
    /// The serial device to receive data from.
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

    println!();

    // TODO: Check received data againgst a stream of this pattern. We might start somewhere in the
    // middle.
    //
    // let pattern = b" 0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~";

    // This read loop is inspired by
    // https://github.com/tosc-rs/mnemos/blob/a0ab8fe0f235f5ef970ec2424926f03ffc887dea/tools/crowtty/src/main.rs#L289.
    loop {
        let mut buffer = [0u8; 256];

        match port.read(&mut buffer) {
            Err(e) if e.kind() == ErrorKind::WouldBlock => continue,
            Err(e) if e.kind() == ErrorKind::TimedOut => continue,
            Err(e) => panic!("{:?}", e),
            Ok(count) => std::io::stdout().write_all(&buffer[..count]).unwrap(),
        }
    }
}
