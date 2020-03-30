# Roid Rage!!!

A silly asteroids game.

## How to run

First you need to run a "pilot" process:

```
cargo run --bin simple-pilot
```

This is the part that calculates where to fire.

Then, you need to start the game engine:

```
cargo run --bin roid_rage
```

This renders everything, manages most of the data, and asks the pilot how to fire.

## Settings

Roid Rage has a number of setting you can modify. These can be controlled via environment variables or a settings file.

TODO: Document these and how to set them.