# CS128H-Final

[IN PROGRESS]

<b>Chess Group</b>

## Authors
Brendan Biernacki (bab8), Andrew Yatzkan (yatzkan3), Chaoyi Xu (chaoyix2), Khalid Alotaibi (khalidaa)

## Project Introduction
Our project involves building a chess game with all the rules including castle and en passant. We chose this project because we are passionate about chess and find building a chess game a suitable project for Rust.

## System Overview
We plan to build a fully functional chess game in Rust. There are two main steps we foresee. The first is constructing the abstraction of the chess board and each of the pieces. We will determine how we can control the movements of the pieces and how to signal for or prevent an invalid move. The other component is implementing a visual interface through which players can submit moves. We expect to use [Pickerel](https://github.com/AidanGlickman/CS128Hon-Final) to help us display the game and allow full play of the game from the user's perspective.

If we have time remaining, we will work on additions such as recording the moves in a score sheet or creating some generalization of chess.

## Possible Challenges
The most obvious challenges can include implementing en passant and castling, as these moves are situation-specific. Additionally, if we want to create a visual component for the user to input moves, such as clicking on squares, this may be tricky to coordinate across all components of the project. We should expect frequent testing throughout the building process

## References
- https://github.com/AidanGlickman/CS128Hon-Final
