# moonrender

Moonrender is the rendering engine for "the small web", used in Moonlander.

Moonrender uses `cairo` and `pango` for rendering.

Moonrender currently supports the following mimetypes:

- `text/gemini`
- `text/plain`

## Example

```rust
let render = moonrender::Renderer::new(moonrender::Theme::default());

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

let scroll_y = 0.0;
let height = 720.0;

let (w, h) = render.draw(scroll_y, height, ctx);
// w and h are the dimensions of the resulting page

// Send a click event to the page
let msg: moonrender::Msg = render.click((x, y));

// reset the renderer state, needed for page navigation etc.
render.reset();
```
