# tic-tac-toe
Tic Tac Toe game with online multiplayer

# Setup
The setup consists of two clients and a server that keeps track of the game state.

Said server is independent from the clients, meaning that it could theoretically be deployed 
with a simple scripts that creates a new server when it receives a request.

## Instructions
1. Run the server:
  - Run `./tic-tac-toe --server true`
  - You can specify the port using `--port <port>`. The default one is 3333.
  - You can change the size of the board with `--size <size>`
  
2. Run the two clients:
  - Run `./tic-tac-toe --server false`
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
