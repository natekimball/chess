# Chess

Chess game written in Rust complete with all the rules of chess, including castling, en passant, pawn promotion, check/checkmate detection, and the fifty-move rule. The game allows you to play against another player or our custom AI chess algorithm.

## Play

AI gameplay

```shell
cargo run
```

Two-player gameplay

```shell
cargo run --2p
```

## Algorithm Design

First, the chess AI was trained on stockfish data, to build a model that could roughly evaluate board states. To make decisions, the algorithm performs a mini-max search with alpha-beta pruning to a depth of _. This didn't produce very good results because it would require days and days of compute time to learn the intricacies of chess from stockfish data. Next, the algorithm was further trained via the amplification technique, where the model is trained on its own output after performing a mini-max search. This guarantees convergence on game theory optimal strategy, because as the model improves, its amplified self will also improve.

## Training the AI

```shell
python model/train.py
```

### Training the AI on Rivanna
  
```shell
sbatch model/train.slurm
```
