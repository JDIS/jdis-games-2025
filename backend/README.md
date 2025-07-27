# FireWall Backend - JDIS Games 2025

## Usage

To run the server, all you need is a working Rust install.  
Then, simply run :

```bash
cargo run
```

## Architecture

The game server is split in 3 parts.

### WebSocket Server

All the server code is contained inside the [`server`](./src/server) folder.  
This part contains everything needed for the clients to communicate with the game. The server is also the entity that contains the game, the client, etc.

### Game Logic

All the game logic is contained inside the [`game`](./src/game) folder.  

### CLI logic

This part reads from `stdin` and executes the user's commands. All the logic is contained inside [`console.rs`](./src/console.rs).
