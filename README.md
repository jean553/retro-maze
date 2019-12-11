# Retro Maze

![Screenshot](screenshot.png)

**This is a work in progress...**

A simple game in Rust. You control a car from a departure to an arrival. The car continuously follows the road until it meets an intersection. At an intersection, you choose the next direction to follow. The car must pass through all the checkpoints before going to the arrival.

## Information
 * Developed in Rust with Piston
 * Graphics created with Blender

## Build the environment

```sh
vagrant up
```

## Connect to the environment

```sh
vagrant ssh
```

## Build the project

```sh
cargo build --release
```

## Run the project

```sh
./target/release/retro-maze
```
