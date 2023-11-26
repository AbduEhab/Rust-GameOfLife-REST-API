# Overview

This is a simple 3D implementation of the [Conway's Game of Life](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life) in Rust. It has been created as a proof of concept for a personal project.

## Usage

You would need to run both the server and the client. The server is a simple REST API that takes the ammount of neighbors and returns the next state of the cell. The client is a simple 3D implementation of the game that uses the server to calculate the next state of the cells.

### Server

To run the server you would need to run the following command:

```bash
cargo run --bin rest-api --release
```

### Client

To run the client you would need to run the following command:

```bash
cargo run --bin game-of-life --release
```

## Shortcomings

The game initialises with a random state. This means that the game can start with a state that is not stable. This can be fixed by adding a way to initialise the game with a stable state. Maybe by adding a way to load a state from a file. This often results in the game reaching a static state after a few iterations.
