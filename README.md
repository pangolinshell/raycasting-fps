# Raycasting FPS

A small multiplayer FPS prototype written in Rust, using a classic raycasting rendering technique inspired by early 3D games.

The project is split into two binaries:

- `server`: hosts the multiplayer session and loads the map.
- `client`: connects to a server, renders the game, and lets the player move around.

## Features

- Real-time raycasting renderer
- Multiplayer client/server architecture
- UDP-based networking
- JSON map loading
- Texture-based walls and entities
- Minimap display
- Basic player movement and collision
- Shooting mechanic
- Configurable server port and map file
- Nickname-based client connection

## Tech Stack

- Rust
- SDL2
- SDL2_image
- SDL2_ttf
- Serde
- serde_json
- Clap

## Project Structure

```txt
.
├── assets/
│   ├── fonts/
│   └── img/
├── conf/
│   ├── map1.json
│   ├── map2.json
│   └── map_template_rework.jsonc
├── src/
│   ├── client/
│   ├── display/
│   ├── entities/
│   ├── resources/
│   ├── server/
│   ├── world/
│   ├── camera.rs
│   ├── rays.rs
│   └── lib.rs
├── Cargo.toml
└── README.md
```

## Requirements

You need Rust and Cargo installed.

You also need the SDL2 native libraries installed on your system.

### Linux

For Debian or Ubuntu-based distributions:

```bash
sudo apt install libsdl2-dev libsdl2-image-dev libsdl2-ttf-dev
```

### macOS

Using Homebrew:

```bash
brew install sdl2 sdl2_image sdl2_ttf
```

## Build

```bash
cargo build --release
```

## Run

### Start the server

```bash
cargo run --release --bin server -- --map "conf/map1.json" --port 5000
```

This starts a server using the map file `conf/map1.json` on port `5000`.

You can also use another map:

```bash
cargo run --release --bin server -- --map "conf/map2.json" --port 5000
```

### Start a client

Open another terminal and run:

```bash
cargo run --release --bin client -- --host 127.0.0.1 --port 5000 --nickname "guest1"
```

To connect another player, open a new terminal and use another nickname:

```bash
cargo run --release --bin client -- --host 127.0.0.1 --port 5000 --nickname "guest2"
```

## Controls

| Key | Action |
| --- | --- |
| `W` | Move forward |
| `S` | Move backward |
| `A` | Strafe left |
| `D` | Strafe right |
| `Q` | Rotate left |
| `E` | Rotate right |
| `Space` | Shoot |
| `Esc` | Quit |

## Server Options

```bash
cargo run --release --bin server -- [OPTIONS]
```

Available options:

| Option | Description |
| --- | --- |
| `--map <FILE>` | Map file to load |
| `--port <PORT>` | Server port |
| `--max-hosts <NUMBER>` | Maximum number of connected players |

Example:

```bash
cargo run --release --bin server -- --map "conf/map1.json" --port 5000 --max-hosts 4
```

## Client Options

```bash
cargo run --release --bin client -- [OPTIONS]
```

Available options:

| Option | Description |
| --- | --- |
| `--host <IP>` | Server IP address |
| `--port <PORT>` | Server port |
| `--nickname <NAME>` | Player nickname |

Example:

```bash
cargo run --release --bin client -- --host 127.0.0.1 --port 5000 --nickname "guest1"
```

## Map Format

Maps are stored as JSON files inside the `conf/` directory.

A map contains:

- `layout`: the tile grid of the map.
- `textures_bindings`: links between tile values and texture names.
- `spawnpoints`: available player spawn positions.
- `resources`: texture and font paths.

Example:

```json
{
  "layout": [
    [1, 1, 1],
    [1, 0, 1],
    [1, 1, 1]
  ],
  "placeholder": "placeholder",
  "textures_bindings": {
    "1": "redbrick"
  },
  "spawnpoints": [
    {
      "x": 1,
      "y": 1
    }
  ],
  "resources": {
    "textures_directory": "assets/img/",
    "font_directory": "assets/fonts/",
    "textures": {
      "redbrick": "redbrick.png",
      "placeholder": "placeholder.png"
    },
    "fonts": {}
  }
}
```

## Notes

This project is a learning-oriented prototype.

It focuses on implementing low-level game programming concepts such as:

- raycasting
- map parsing
- SDL2 rendering
- multiplayer synchronization
- UDP networking
- basic entity management

It is not meant to be a complete production-ready FPS engine.

## Possible Improvements

- Add mouse-based camera rotation
- Add health and score display
- Improve shooting feedback
- Add proper player death and respawn delay
- Add menu and lobby system
- Add sound effects
- Improve network reliability
- Add server-side anti-cheat checks
- Add more maps and entities

## License

No license has been specified yet.