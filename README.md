# Smartstore Organizer WEb

Web UI for [Smartstore Box Organizer](https://github.com/hasanen/smartstore-box-organizer-generator)

## Setup

1. Install trunk

```bash
cargo install trunk
```

2. Add wasm target

```bash
rustup target add wasm32-unknown-unknown
```

## Dev server

```bash
trunk serve
```

## Build prod

```bash
trunk build --release
```
