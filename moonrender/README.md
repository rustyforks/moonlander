# moonrender

Moonrender is the rendering engine for "the small web", used in Moonlander.

Moonrender uses `cairo` and `pango` for rendering.

Moonrender currently supports the following mimetypes:

- `text/gemini`

## Example

```rust
let render = moonrender::Renderer::new();

// optional, required for relative link handling
render.set_url("gemini://ecmelberk.com");

// set page type
render.set_mime("text/gemini");

// can repeat this to slowly build the page, useful for streaming
// important: end with new line to complete the page.
render.new_page_chunk("""
# text/gemini content

=> gemini://test.test Test
""");

let ctx: cairo::Context = ...;
let (w, h) = render.draw(ctx);

// Send a click event to the page
let msg: moonrender::Msg = render.click((x, y));

// reset the renderer state, needed for page navigation etc.
render.reset();
```
