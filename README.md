# Roid Rage!!!

A silly asteroids game.

## How to run

Roid Rage runs as two processes, a *game* process and a *pilot* process. Generally you'll start
the game process first. When you subsequently start pilot processes, they will register with
the game process to start receiving requests for commands.
### The game process

You can start the game process like this:
```
cargo run --bin roid-rage
```

This command will create a new window and you should see 'roids floating around the screen. At this point
you won't see a ship on the screen. The ship will get created when you start a pilot process and it registers
with the game.

### The "manual" pilot

The `manual-pilot` pilot process lets you actually play the game yourself (i.e. it's not a bot). It accepts
keyboard input and uses that to determine how the ship moves. Start it like this:

```
cargo run --bin manual-pilot
```

The terminal in which you ran `manual-pilot` is the program that will take your input for piloting
the ship. Focus on that terminal and control the ship as follows:

* Left arrow - rotate counterclockwise
* Right arrow - rotate clockwise
* Up arrow - engage thrusters
* Space bar - fire cannon
* S - bring the ship to a stop

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

Currently the best example of how to implement a pilot is `manual-pilot`. You can use
this as a rough guide of how to write a pilot.

## Settings

Roid Rage has a number of setting you can modify. These can be controlled via environment variables or a settings file.

TODO: Document these and how to set them.