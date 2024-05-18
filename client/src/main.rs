use std::io::{self, ErrorKind, Read, Write};
use std::net::TcpStream;
use std::sync::mpsc::{self, TryRecvError};
use std::thread;
use std::time::Duration;

const LOCAL: &str = "127.0.0.1:6000";
const MSG_SIZE: usize = 32;

fn main() {
    let client = TcpStream::connect(LOCAL).expect("Stream failed to connect");
    client.set_nonblocking(true).expect("Failed to initiate non-blocking");

    let (tx, rx) = mpsc::channel::<String>();

    // Spawn a thread to handle communication with the server
    thread::spawn(move || handle_server_communication(client, rx));

    println!("Write a Message:");
    handle_user_input(tx);
    println!("bye bye!");
}

/// Handles communication with the server.
fn handle_server_communication(mut client: TcpStream, rx: mpsc::Receiver<String>) {
    loop {
        if let Err(_) = read_from_server(&mut client) {
            println!("Connection with server was severed");
            break;
        }

        if let Err(_) = write_to_server(&mut client, &rx) {
            break;
        }

        thread::sleep(Duration::from_millis(100));
    }
}

/// Reads messages from the server.
fn read_from_server(client: &mut TcpStream) -> Result<(), ()> {
    let mut buff = vec![0; MSG_SIZE];
    match client.read_exact(&mut buff) {
        Ok(_) => {
            let msg = buff.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>();
            println!("Message received: {:?}", msg);
            Ok(())
        },
        Err(ref err) if err.kind() == ErrorKind::WouldBlock => Ok(()),
        Err(_) => Err(()),
    }
}

/// Writes messages to the server from the channel.
fn write_to_server(client: &mut TcpStream, rx: &mpsc::Receiver<String>) -> Result<(), ()> {
    match rx.try_recv() {
        Ok(msg) => {
            let mut buff = msg.clone().into_bytes();
            buff.resize(MSG_SIZE, 0);
            client.write_all(&buff).expect("Writing to socket failed");
            println!("Message sent: {:?}", msg);
            Ok(())
        },
        Err(TryRecvError::Empty) => Ok(()),
        Err(TryRecvError::Disconnected) => Err(()),
    }
}

/// Handles user input and sends it through the channel.
fn handle_user_input(tx: mpsc::Sender<String>) {
    loop {
        let mut buff = String::new();
        io::stdin().read_line(&mut buff).expect("Reading from stdin failed");
        let msg = buff.trim().to_string();
        if msg == ":quit" || tx.send(msg).is_err() {
            break;
        }
    }
}
