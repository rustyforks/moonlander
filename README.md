# moonlander

Moonlander is the fanciest Gemini client in the entire solar system.

## Building

```bash
cargo build --release
cd target/release
strip -s moonlander # optional, reduces file size by ~50%
```

### Requirements

- GTK 3
- Cairo
- Pango
- A new-ish Rust compiler

#### Windows

See [Gtk-rs Windows requirements](http://gtk-rs.org/docs/requirements.html#windows)

## Configuration

Run Moonlander for the first time to create the configuration defaults.

- **Windows:** `%APPDATA%/ecmelberk/moonlander/config.toml`
- **Linux:** `$XDG_CONFIG_HOME/moonlander/config.toml` (`$XDG_CONFIG_HOME` is
  `$HOME/.config` under most cases)
- **macOS:** `$HOME/Library/Preferences/com.ecmelberk.moonlander/config.toml`

## Embedding

If you want to embed Moonlander's rendering engine in your own application, see
the [`moonrender`](./moonrender) directory. If your application uses [relm], you
can use [`relm-moonrender`](./relm-moonrender).

[relm]: https://github.com/antoyo/relm
