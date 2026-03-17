# Rust Invaders

A terminal-based Space Invaders clone written in Rust, built as a learning project.

## Features

- Terminal rendering using [crossterm](https://github.com/crossterm-rs/crossterm)
- Sound effects via [rusty_audio](https://github.com/CleanCut/rusty_audio)
- Multithreaded render loop
- Invaders that move and speed up as they are killed
- Player shooting with up to 2 simultaneous shots

## Controls

| Key | Action |
|-----|--------|
| `←` / `→` | Move left / right |
| `Space` or `Enter` | Shoot |
| `Esc` | Quit |

## Building and Running

```bash
# Build
cargo build

# Run
cargo run
```

## Project Structure

```
src/
  main.rs       - Entry point, game loop, input handling
  lib.rs        - Constants (NUM_ROWS, NUM_COLS)
  frame.rs      - Frame buffer definition
  render.rs     - Terminal rendering
  player.rs     - Player movement and shooting
  invaders.rs   - Invader army movement and collision
  shot.rs       - Projectile logic
audio/          - WAV sound effects (multiple audio packs available)
```

## Dependencies

- `crossterm` — cross-platform terminal control
- `rusty_audio` — audio playback
- `rusty_time` — timer utilities