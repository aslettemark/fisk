# fisk
Rust chess engine using bitboards, leveraging the fact that you can squeeze a lot of
chess board state into a couple of 64 bit words, and that you can do a lot of really
fast operations on these words on modern 64 bit CPUs. 

[BitBoards (Wikipedia)](https://en.wikipedia.org/wiki/Bitboard)  
[BitBoards (chessprogramming.org)](https://www.chessprogramming.org/Bitboards)  
[Forsyth–Edwards Notation](https://en.wikipedia.org/wiki/Forsyth%E2%80%93Edwards_Notation)   
