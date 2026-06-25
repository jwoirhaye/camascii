# camascii

Live webcam stream rendered as colored ASCII art in your terminal.

## Requirements

- Linux (V4L2)
- A webcam available at `/dev/video0`
- A terminal with true color support (kitty, alacritty, gnome-terminal, …)

```bash
sudo apt install libv4l-dev
```

## Build

```bash
cargo build --release
```

The binary is at `target/release/camascii`.

## Usage

```bash
./target/release/camascii
```

| Key | Action |
|-----|--------|
| `q` | Quit |
| `Esc` | Quit |
| `Ctrl+C` | Quit |

## Project structure

```
src/
├── main.rs    # Terminal init and main loop
├── app.rs     # Application state and keyboard handling
├── camera.rs  # V4L2 camera wrapper
├── codec.rs   # YUYV decoding and luminance-to-ASCII conversion
└── ui.rs      # Ratatui widgets and layout
```

## License

[MIT](LICENSE)