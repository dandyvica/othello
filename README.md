# Othello game engine in Rust
This is an experiment in order to build an Othello game engine in Rust, and later on to communicate with Javascript using WebAssembly.

## Othello
See https://en.wikipedia.org/wiki/Reversi for game rules. These are very simple, but not straightforward when it comes to programming a game engine. 

## Bitboards
For board games like chess, checkers or othello, the board representation is usually called a `bitboard`. For Othello, because it's a 8x8 board dimension, black or white pieces (or disks) are kept using a 64-bits integer (`u64` type in Rust) for one player, with bits set to 1 for his/her pieces, and 0 for empty squares. Another bitboard is necessary to keep opposite color pieces.

It's a linear representation for the 8x8 board: the most significant bit is bit 63 and it's corresponding to the **A1** square, while the least significant bit 0 is **H8**:

![Othello board with no pieces](./doc/empty_board.png)

It's interesting to note that if:
```rust
black: u64;     // bits set to '1' for occupied squares
white: u64;     // and to 0 for empty ones
```

then:

```rust
(black & white).count_zeros()
```

gives the number of empty squares (ref. to the **POPNCNT** X64_64 assembly instruction).



