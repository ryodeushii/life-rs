# Rust implementation of *Conway's Game of Life*

Building is possible using either makefile or docker.

Default params for executable (if no params will be provided):
* Grid width: 25
* Grid height: 25
* Grid is bordered (non circular plane)
* FPS is 20


Executable params:
```
Conway's Game of Life implementation in rust

Usage: life [OPTIONS]

Options:
  -W, --width <width>    Width of the grid [default: 25]
  -H, --height <height>  Height of the grid [default: 25]
  -F, --fps <fps>        Frames per second [default: 20]
  -C, --circular         Circular plane grid
  -h, --help             Print help
  -V, --version          Print version
```

If -C or --circular flag is set - grid will be assumed as circular plane grid (if neighbors are "outside" of borders we go on opposite side and calculate it as neighbor).

