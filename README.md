# Roid Rage!!!

A silly asteroids game.

## How to run

Roid Rage runs as two processes, a *game* process and a *pilot* process. You can start them in
any order.

If you want to actually play the game yourself, you'll want to use the `manual-pilot` pilot 
program:
```
cargo run --bin manual-pilot
```

Then you can start the game process:
```
cargo run --bin roid-rage
```

This second command will create a new window and you should see 'roids floating around the screen with your
ship in the middle.

The terminal you ran `manual-pilot` in is the program that will take your input for piloting
the ship. Focus on that terminal (while keeping the `roid-rage` window visible!) and control the ship
as follows:

* Left arrow - rotate counterclockwise
* Right arrow - rotate clockwise
* Up arrow - engage thrusters
* Space bar - fire cannon

### Other pilots

One of the goals of Roid Rage is to support development of autopilots, i.e. programs
that pilot the ship. We currently have two examples of (very, very stupid) autopilots, `driver-pilot` and
`simple-pilot`. You can use these instead of `manual-pilot` like by replacing 'manual-pilot' in the
first command above with either 'simple-pilot' or 'driver-pilot', e.g.:

```
cargo run --bin simple-pilot
```

or

```
cargo run --bin driver-pilot
```

### Developing pilots

A pilot is just a program that receives requests from the `roid-rage` process for commands. The requests
include information about the ship, it's location, and the 'roids on the board. It responds with
instructions about how to pilot the ship, whether to fire, etc. Communication takes place
using [grpc](grpc.io).

Currently the best example of how to implement a pilot is `driver-pilot`. You can use
this as a rough guide of how to write a pilot.

## Settings

Roid Rage has a number of setting you can modify. These can be controlled via environment variables or a settings file.

TODO: Document these and how to set them.