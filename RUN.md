# How to run this project

## Clone the project
Try one of these two commands: (one is for HTTPS and one is for SSH)
```bash
git clone https://github.com/brendanb321/CS128H-Final
git clone git@github.com:brendanb321/CS128H-Final.git
```
## Navigate to project directory
This can be done manually or by executing the command
```bash
cd CS128H-Final
```

## Build the project
```bash
cargo build
```

## Run the project
```bash
cargo run
```

## Instructions
The game will now appear in your terminal window. For each move, you will be asked to separately enter the coordinates of the piece you would like to move and the coordinates to where you would like to move the piece. If any input is invalid, no move will be made and it will remain that player's turn. (The number in the error message is used for debugging.) If both inputs are valid, then the piece will be moved accordingly and the turn will switch. The instructions in the terminal should be relatively self-explanatory.

