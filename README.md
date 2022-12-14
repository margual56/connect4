# Connect 4
Connect 4 game with online multiplayer with an arbitrary board size

# Setup
The setup consists of two clients and a server that keeps track of the game state.

Said server is independent from the clients, meaning that it could theoretically be deployed 
with a simple scripts that creates a new server when it receives a request.

## Releases
I have cross-compiled the game for windows, linux gnu and linux musl.
Go to [the releases page](https://github.com/margual56/tic-tac-toe/releases) and download the one you want to use!

## Instructions
1. Run the server:
  - Run `./connect4 --server true`
  - You can specify the port using `--port <port>`. The default one is 3333.
  - You can change the size of the board with `--size <size>`
  
2. Run the two clients:
  - Run `./connect4 --server false`
  - You can specify the IP with `--ip <ip>`. The default is localhost.
  - You can specify the port using `--port <port>`. The default one is 3333.
  
3. Play!

# Technical details
This uses TCP to establish two connections: one connection for each player.

On that connection, the server sends status codes to the clients to communicate to them the state of the game (if it is their turn, if they won/lost, etc).
When is the turn of a player, the server waits to receive a position to drop the chip. 
If the position is not valid, it does not drop the chip and tells the player that it is its turn again.

If a client errors out and/or disconnects, the server and other client will crash. 

# Contribute
Please, contribute! Additions, fixes and improvements are more than welcome :D
