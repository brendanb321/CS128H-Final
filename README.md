# Chess Group
https://github.com/brendanb321/CS128H-Final.git

Final project for CS 128 Honors at UIUC.

A simple chess game written in Rust.

## Authors
Brendan Biernacki (bab8), Andrew Yatzkan (yatzkan3), Chaoyi Xu (chaoyix2), Khalid Alotaibi (khalidaa)

## Project Introduction
Our project involves building a chess game with all the rules including castle and en passant. We chose this project because we are passionate about chess and find building a chess game a suitable project for Rust.

## System Overview
We plan to build a fully functional chess game in Rust. There are two main steps we foresee. The first is constructing the abstraction of the chess board and each of the pieces. We will determine how we can control the movements of the pieces and how to signal for or prevent an invalid move. The other component is implementing a visual interface through which players can submit moves. We expect to use some Rust graphics library like [quicksilver](https://github.com/ryanisaacg/quicksilver) to help us display the game and allow full play of the game from the user's perspective.

If we have time remaining, we will work on additions such as recording the moves in a score sheet or creating some generalization of chess. We may also consider combining this with [Pickerel](https://github.com/AidanGlickman/CS128Hon-Final) to visualize the algorithm implemented in this project.

## Possible Challenges
The most obvious challenges can include implementing en passant and castling, as these moves are situation-specific. Additionally, if we want to create a visual component for the user to input moves, such as clicking on squares, this may be tricky to coordinate across all components of the project. We should expect frequent testing throughout the building process to ensure all moves allowed are precisely the valid chess moves.

## References
- https://github.com/AidanGlickman/CS128Hon-Final
- https://github.com/ryanisaacg/quicksilver
