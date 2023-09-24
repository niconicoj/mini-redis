use std::{
    collections::HashMap,
    net::{TcpListener, TcpStream},
};

use mini_redis::{send_response, Error, Request, Response, BIND_ADDRESS};

pub fn main() {
    let listener = TcpListener::bind(BIND_ADDRESS).expect("failed to bind to address");

    let mut store: HashMap<String, String> = HashMap::new();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => handle_connection(stream, &mut store),
            Err(_) => eprintln!("failed to handle incoming connection"),
        }
    }
}

fn handle_connection(mut stream: TcpStream, store: &mut HashMap<String, String>) {
    let result = rmp_serde::decode::from_read::<_, Request>(&stream)
        .map(|request| handle_request(request, store, &mut stream));
    if result.is_err() {
        send_response(Response::Failure(result.unwrap_err().into()), &mut stream).unwrap()
    }
}

fn handle_request(
    request: Request,
    store: &mut HashMap<String, String>,
    stream: &mut TcpStream,
) -> Result<(), Error> {
    match request {
        Request::Write(key, value) => {
            store.insert(key, value);
            send_response(Response::Success(None), stream).unwrap();
            Ok(())
        }
        Request::Read(key) => {
            match store.get(&key) {
                Some(value) => {
                    send_response(Response::Success(Some(value.clone())), stream).unwrap();
                }
                None => {
                    send_response(Response::Failure(Error::NotFound), stream).unwrap();
                }
            };
            Ok(())
        }
        Request::Delete(key) => {
            store.remove(&key);
            send_response(Response::Success(None), stream).unwrap();
            Ok(())
        }
    }
}
