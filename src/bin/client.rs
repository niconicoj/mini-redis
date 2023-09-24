use std::net::TcpStream;

use clap::{Parser, Subcommand};
use mini_redis::{send_request, Error, Request, BIND_ADDRESS};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Write { key: String, value: String },
    Read { key: String },
    Delete { key: String },
}

impl Into<Request> for Commands {
    fn into(self) -> Request {
        match self {
            Commands::Write { key, value } => Request::Write(key, value),
            Commands::Read { key } => Request::Read(key),
            Commands::Delete { key } => Request::Delete(key),
        }
    }
}

pub fn main() -> Result<(), Error> {
    let cli = Cli::parse();
    let mut client = TcpStream::connect(BIND_ADDRESS).expect("failed to connect to address");
    match send_request(cli.command.into(), &mut client)? {
        mini_redis::Response::Success(Some(value)) => {
            println!("{value}");
            Ok(())
        }
        mini_redis::Response::Success(None) => Ok(()),
        mini_redis::Response::Failure(err) => Err(err),
    }
}
