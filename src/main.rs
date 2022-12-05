use std::env;
use std::net::TcpListener;
use wdms2prod::build_and_start_server;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let port = match args.len() {
        1 => "8000",
        _ => &args[1],
    };
    let listener = TcpListener::bind(format!("127.0.0.1:{}", &port))?;
    build_and_start_server(listener, None)?.await
}
