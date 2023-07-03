use clap::Parser;

#[derive(Debug, Parser)]
pub struct AppConfig {
    /// The baud rate to send data at.
    #[arg(long, default_value_t = 115200)]
    baud: u32,
    /// The serial device to send data with.
    device: String,
}


fn main() {
    let config = AppConfig::parse();
    let mut port = serialport::new(config.device, config.baud)
        .open()
        .unwrap();

    let pattern = b" 0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~";

    loop {
        port.write_all(pattern).unwrap();
    }
}
