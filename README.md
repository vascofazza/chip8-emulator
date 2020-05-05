# chip8-emulator
A Chip-8 emulator written in Rust

## Introduction

This is a chip-8 VM built in rust. If you're reading this, chances are that you're thinking of writing your own emulator. You should! It gives you a great feel for how home computers worked back in the late 70s. 

## Requirements

You need to have sdl2 installed with headers. 

On Linux, run this:

```
sudo apt-get install libsdl2-dev libsdl2-gfx-dev
```

While on Mac:

```
brew install sdl2
```

## Usage

Clone this repository, then run:

```
cargo run /path/to/game
```

You can find public-domain games [here](https://www.zophar.net/pdroms/chip8/chip-8-games-pack.html).

## How to play

The entire chip8 keyboard is mapped like this:

Chip8 keyboard:

|      |      |      |      |
| ---- | ---- | ---- | ---- |
| 1    | 2    | 3    | C    |
| 4    | 5    | 6    | D    |
| 7    | 8    | 9    | E    |
| A    | 0    | B    | F    |

Is mapped to:

|      |      |      |      |
| ---- | ---- | ---- | ---- |
| 1    | 2    | 3    | 4    |
| Q    | W    | E    | R    |
| A    | S    | D    | F    |
| Z    | X    | C    | V    |

### Tetris controls:

- Q - Rotate piece
- W - Move left
- E - Move right
- A - Fast drop

### Invaders controls:

- Q - Move left
- E - Move right
- W - Shoot weapon
