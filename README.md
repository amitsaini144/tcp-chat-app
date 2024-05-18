# Simple TCP Chat Application

This is a simple TCP chat application implemented in Rust, allowing for one-to-one client-server communication. The server can handle multiple clients, but each client operates in its own thread.

## Features

- The server accepts multiple client connections.
- Each client runs in its own thread.
- Clients can send and receive messages from the server.
- Non-blocking I/O for efficient message handling.

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) installed on your machine.

## Getting Started

### Cloning the Repository

```sh
git clone https://github.com/amitsaini144/tcp-chat-app.git
cd tcp-chat-app
```

### Running the Server

1. Open a terminal window.
2. Navigate to the 'swever' directory:

```sh
cd server
cargo run
```

The server will start and listen for connections on '12.0.0.1:600'.

### Running the Client

1. Open a new terminal window.
2. Navigate to the 'client' directory:

```sh
cd client
cargo run
```

3. Enter your message in the client terminal. To exit, type ':quit'.

## Project Structure 

- `client/`: Contains the client application code.
- `server/`: Contains the server application code.
- `Cargo.toml`: Configuration file for Rust's package manager,specifying dependencies and project metadata.

## Code Overview
### Server Code(`server/main.rs`)

The server accepts incoming connections and spawns a new thread to handle each client. Messages received from clients are broadcasted to all connected clients.

### Client Code(`client/main.rs`)

The client connects to the server and allows the user to send messages. It also listens for messages from the server and displays them.


