# moonlander

Moonlander is the fanciest Gemini client in the entire solar system.

**Note:** Moonlander is still under development. While most features work well
enough, issues might happen.

## Features

- Custom, themeable rendering engine via Cairo & Pango
- Tries to follow Gnome HIG

### Known Bugs

- TLS Certificates aren't verified
- Somewhat high resource usage (for a Gemini client)
- Cannot navigate backwards through redirections
- Renderer doesn't behave "native"
  - Cannot select/copy text
  - No interaction other than mouse clicks on links and scrolling

### Planned Features

- Tabs
- Render more than just text/gemini and plaintext.
  - Planned: Markdown & images

- Possibly support other protocols
  - Gopher, etc.
  - Definitely not HTTP, unless excluding HTML

- Syntax highlighting (?)
  - Waiting on text/gemini preformatting annotations to be somewhat standardized

## Building

```bash
git clone https://git.sr.ht/~admicos/moonlander
cd moonlander

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

## Patches, Bug Reports & Discussion

- If you found any issues, please [submit a ticket] (`~admicos/moonlander@todo.sr.ht`).
- For questions, patches or discussion, [send an email] (`~admicos/projects@lists.sr.ht`).

For patches sent via email, please make sure you indicate that it's for Moonlander
by running the following on your clone of the repo:

```sh
git config format.subjectprefix "PATCH moonlander"
```

[submit a ticket]: https://todo.sr.ht/~admicos/moonlander
[send an email]: https://lists.sr.ht/~admicos/projects

## License

Moonlander, Moonrender, relm-moonrender and the Gemini client under the `gemini/`
directory are all licensed under the MIT License, which is available in the [`LICENSE`]
file of this repository.

[`LICENSE`]: ./LICENSE
