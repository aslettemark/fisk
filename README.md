# fisk
Rust chess engine using bitboards, leveraging the fact that you can squeeze a lot of
chess board state into a couple of 64 bit words, and that you can do a lot of really
fast operations on these words on modern 64 bit CPUs. 

You can play against the engine on [Lichess](https://lichess.org/@/Fisk_Bot/).

[BitBoards (Wikipedia)](https://en.wikipedia.org/wiki/Bitboard)  
[BitBoards (chessprogramming.org)](https://www.chessprogramming.org/Bitboards)   
