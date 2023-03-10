# Conway's Game of Life in Rust (TDD)

This is my implementation of Conway's game of life using test driven development in the Rust language.

## Live Demo
[gameoflife.edgardocarreras.com](https://gameoflife.edgardocarreras.com)

### Problem Statement 

Your task is to write a program to calculate the next generation of Conway's game of life, given any starting position.

You start with a two-dimensional grid of cells, where each cell is either alive or dead. 
The grid is finite, and no life can exist off the edges.
When calculating the next generation of the grid, follow these four rules:

1. Any live cell with fewer than two live neighbours dies, as if caused by underpopulation.
2. Any live cell with more than three live neighbours dies, as if by overcrowding.
3. Any live cell with two or three live neighbours lives on to the next generation.
4. Any dead cell with exactly three live neighbours becomes a live cell.

See example/1 and example/2


### Examples
Example 1:
```
* indicates live cell
. indicates dead cell

Example input:
4 8
........
....*...
...**...
.....*..

Example output:
4 8
........
...**...
...***..
....*...

```

Example 2:
```
* indicates live cell
. indicates dead cell

Example input:
5 8
........
...**...
.*****..
........
........

Example output:
5 8
........
.....*..
..*..*..
..***...
........

```


## Environment

Dependencies:
- rustup
- rustup target install wasm32-unknown-unknown
- `cargo install -f wasm-bindgen-cli`


## Build

- Build native release build
```sh
cargo build --release 
```
- Build wasm release build
```sh
cargo build --release --target wasm32-unknown-unknown
```
- Build wasm js bindings
```sh 
wasm-bindgen --out-dir ./pkg/ --target web ./target/wasm32-unknown-unknown/release/game_of_life.wasm
```

- Build web (root: /www)
```shell
npm run build
```
