use std::io::{ErrorKind, Read, Write};
use std::net::TcpListener;
use std::sync::mpsc::{self, Sender};
use std::thread;
use std::time::Duration;

const LOCAL: &str = "127.0.0.1:6000";
const MSG_SIZE: usize = 32;
const SLEEP_DURATION: Duration = Duration::from_millis(100);

fn sleep() {
    thread::sleep(SLEEP_DURATION);
}

fn handle_client(mut socket: std::net::TcpStream, addr: std::net::SocketAddr, tx: Sender<String>) {
    loop {
        let mut buff = vec![0; MSG_SIZE];
        match socket.read_exact(&mut buff) {
            Ok(_) => {
                let msg = buff.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>();
                let msg = String::from_utf8(msg).expect("Invalid utf8 message");

                println!("{}: {:?}", addr, msg);
                tx.send(msg).expect("failed to send msg to rx");
            },
            Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
            Err(_) => {
                println!("closing connection with: {}", addr);
                break;
            }
        }
        sleep();
    }
}

fn main() {
    let server = TcpListener::bind(LOCAL).expect("Listener failed to bind");
    server.set_nonblocking(true).expect("failed to initialize non-blocking");

    let mut clients = vec![];
    let (tx, rx) = mpsc::channel::<String>();

    loop {
        // Accept new client connections
        if let Ok((socket, addr)) = server.accept() {
            println!("Client {} connected", addr);

            let tx = tx.clone();
            clients.push(socket.try_clone().expect("failed to clone client"));

            // Spawn a new thread to handle the client
            thread::spawn(move || handle_client(socket, addr, tx));
        }

        // Broadcast messages to all clients
        if let Ok(msg) = rx.try_recv() {
            clients = clients.into_iter().filter_map(|mut client| {
                let mut buff = msg.clone().into_bytes();
                buff.resize(MSG_SIZE, 0);

                client.write_all(&buff).map(|_| client).ok()
            }).collect();
        }

        sleep();
    }
}
