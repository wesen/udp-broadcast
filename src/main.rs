use clap::{Parser, Subcommand};
use std::fmt::Formatter;
use std::net::UdpSocket;
use std::time::Duration;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

const DEFAULT_PORT: u16 = 5555u16;

#[derive(Subcommand, Debug)]
enum Commands {
    Client {
        #[clap(short, long, value_parser)]
        broadcast: bool,
        #[clap(default_value_t = String::from("255.255.255.255"), short, long, value_parser)]
        address: String,
        #[clap(default_value_t = DEFAULT_PORT, short, long, value_parser)]
        port: u16,
    },
    Server {
        #[clap(default_value_t = false, short, long, value_parser)]
        broadcast: bool,
        #[clap(default_value_t = String::from("0.0.0.0"), short, long, value_parser)]
        address: String,
        #[clap(default_value_t = DEFAULT_PORT, short, long, value_parser)]
        port: u16,
    },
}

#[derive(Debug)]
enum Error {
    IoError(std::io::Error),
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::IoError(err)
    }
}

fn run_client(broadcast: bool, address: &str, port: u16) -> Result<(), Error> {
    let socket: UdpSocket = UdpSocket::bind("0.0.0.0:0")?;
    socket.set_broadcast(broadcast)?;
    println!(
        "broadcast: {}, local addr: {}",
        socket.broadcast().unwrap(),
        socket.local_addr().unwrap()
    );

    let data = [1u8, 2, 3, 4, 5];
    let n = socket.send_to(&data, (address, port))?;
    println!("Sent {} bytes to {}:{}", n, address, port);

    Ok(())
}

fn run_server(broadcast: bool, address: &str, port: u16) -> Result<(), Error> {
    let socket: UdpSocket = UdpSocket::bind(format!("{}:{}", address, port))?;

    socket.set_broadcast(broadcast)?;
    println!(
        "broadcast: {}, local addr: {}",
        socket.broadcast().unwrap(),
        socket.local_addr().unwrap()
    );

    let mut buf = [0u8; 64];
    while let (n, addr) = socket.recv_from(&mut buf)? {
        println!("{} bytes from {:?}", n, addr);
    }

    Ok(())
}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Client {
            address,
            port,
            broadcast,
        } => run_client(*broadcast, &address, *port).unwrap(),
        Commands::Server {
            address,
            port,
            broadcast,
        } => run_server(*broadcast, &address, *port).unwrap(),
    }
}
