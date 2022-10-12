# Relearn

This repository contains some game-playing algorithms developed during my (ongoing) study of
artificial intelligence.

The goal is to develop a framework containing interfaces where `Games` and `Players` implementations
can interact and experiment how different `Agents` algorithms fare, from classical to reinforcement
learning.

## Usage

Currently there is no CLI implementation, so the parameters are controlled in some `const` in the
top of the `/src/main.rs` file.

```shell
$ cargo run -r
Win: 84.40%, Draw: 14.00%, Loss: 1.60%, Game Count: 984
```
