use tokio::{net::TcpListener, io::{BufReader, AsyncBufReadExt, AsyncWriteExt}, self};
use p2p_chat::Input;

#[tokio::main]
async fn main() {

    println!("1. Start new network\n2. Join existing network\n(1/2):");

    let opt = Input::read_int();
    
    match opt {
        1 => {
            start_server().await;
        }
        2 => {
            println!("Enter the address: ");
            let ip_addr = Input::read_string();
            // ip_addr.push_str("/hi");

            let body = reqwest::get(ip_addr)
                .await
                .unwrap();

            println!("{:?}",body);
        }
        _ => {
            println!("Invalid input");
        }
    }
}

async fn start_server() {

    // Substitute with better way to create Ip Address
    let ip_addr = "127.0.0.1:8080";
    println!("{ip_addr}");

    // Start server on ip_addr listening for connections
    let listener = TcpListener::bind(ip_addr).await.unwrap();

    loop {

        // Accept connection
        let (mut socket, addr) = listener.accept().await.unwrap();
        println!("{addr}");
        // tokio::spawn(async move {
            let (reader, mut writer) = socket.split();

            let reader = BufReader::new(reader);

            let http_request = reader
                .lines()
                .next_line()
                .await
                .unwrap()
                .unwrap();

            println!("{http_request}");

            let status_line = "HTTP/1.1 200 OK";
            let content = "Hello world";
            let response = format!("{status_line}\r\nContent-Length:{}\r\n\r\n{}", content.len(), content);
            writer.write_all(response.as_bytes()).await.unwrap();
        // });

    }
}


