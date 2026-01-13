# AGENTS

## Overview
Roid Rage is a Rust workspace for a small Asteroids-style game and multiple pilot processes (manual and bot pilots). The game engine (`roid-rage`) runs as one process; pilots connect over gRPC via `roid-rage-grpc`.

## Workspace layout
- `roid-rage/`: Game engine and rendering (ggez), ECS-style components/systems.
- `roid-rage-grpc/`: gRPC definitions and generated code (Tonic); proto in `proto/roid-rage/roid-rage.proto`.
- `pilot-lib/`: Helpers for building pilots; see `pilot_base::pilot_main`.
- `manual-pilot/`: Keyboard-controlled pilot.
- `simple-pilot/`: Example bot pilot.
- `driver-pilot/`: Another example bot pilot.
- `sted/`: Shared math/utility types used by pilots (direction/velocity, etc.).
- `resources/`: Game assets (fonts, licenses).

## Common commands
- Run the game: `cargo run --bin roid-rage`
- Run a pilot: `cargo run --bin manual-pilot` (or `simple-pilot`, `driver-pilot`)
- Tests (workspace): `cargo test --workspace`

## Notes for changes
- Game logic lives in `roid-rage/src/systems/` and component data in `roid-rage/src/components/`.
- Pilot behavior typically lives in the pilot binary crate (e.g., `manual-pilot/src/`, `simple-pilot/src/`).
- gRPC types are generated from `roid-rage-grpc/proto/roid-rage/roid-rage.proto` via `roid-rage-grpc/build.rs`.
- Asset loading for the game uses `resources/` via ggez's `add_resource_path`.
