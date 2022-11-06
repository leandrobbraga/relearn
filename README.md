# Relearn

This repository contains some game-playing algorithms developed during my (ongoing) study of
artificial intelligence.

The goal is to develop a framework containing interfaces where `Games` and `Players` implementations
can interact and experiment how different `Agents` algorithms fare, from classical to reinforcement
learning.

## Usage

The process was split in two steps: learning and playing. That was done because in most agents the
process of learning is orders of magnitude slower than the process of playing. This way it's
possible to cache learned agents to avoid paying the learning cost again.

### Learning

Some agents need to learn ahead of time (e.g., min-max). To do that, run
`cargo run -r learn <PLAYER>`. For example: `cargo run -r learn min-max`.

### Playing

To make the agents play the games, run `cargo run -r play <PLAYER_1> <PLAYER_2> <GAME_COUNT>`

Example:

```shell
$ cargo run -r play min-max random 100
Win: 89.58%, Draw: 10.42%, Loss: 0.00%, Game Count: 96
```
