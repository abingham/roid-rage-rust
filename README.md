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

The `pilot-lib` module is designed to simplify development of new pilots. In particular, it implements a `main()` function
that most pilots should be able to use. 

Currently the best example of how to implement a pilot is `manual-pilot`. You can use
this as a rough guide of how to write a pilot.

## Settings

Roid Rage has a number of setting you can modify. These can be controlled via environment variables or a settings file.

Settings are loaded from an optional `Settings.toml` in the working directory and then
overridden by environment variables prefixed with `ROID_RAGE_`. Environment variables use
the exact setting name in uppercase, for example `screen_width` becomes `ROID_RAGE_SCREEN_WIDTH`.

Example `Settings.toml`:
```toml
screen_width = 1024.0
screen_height = 768.0
pilot_registration_url = "[::1]:50051"
```

Example environment overrides:
```bash
ROID_RAGE_SCREEN_WIDTH=1024 ROID_RAGE_SCREEN_HEIGHT=768 cargo run --bin roid-rage
```

Available settings and defaults (see `roid-rage/src/settings.rs`):

| Setting | Type | Default | Purpose |
| --- | --- | --- | --- |
| screen_width | f32 | 800.0 | Window width. |
| screen_height | f32 | 600.0 | Window height. |
| minimum_roid_radius | f32 | 15.0 | Smallest roid radius. |
| maximum_roid_radius | f32 | 42.5 | Largest roid radius. |
| roid_bumpiness | f32 | 0.1 | Irregularity factor for roid shape. |
| rate_of_fire | f32 | 0.5 | Seconds between ship shots. |
| bullet_speed | f32 | 1000.0 | Bullet speed. |
| min_initial_roid_speed | f32 | 50.0 | Minimum initial roid speed. |
| max_initial_roid_speed | f32 | 100.0 | Maximum initial roid speed. |
| initial_roid_count | u32 | 10 | Roids spawned at startup. |
| ship_length | f32 | 10.0 | Ship length. |
| ship_width | f32 | 5.0 | Ship width. |
| ship_mass | f32 | 1.0 | Ship mass. |
| ship_thrust | f32 | 300.0 | Ship thrust force. |
| ship_rotational_speed | f32 | 6.0 | Ship rotational speed (radians/sec). |
| pilot_registration_url | String | "[::1]:50051" | gRPC registration listener address. |
