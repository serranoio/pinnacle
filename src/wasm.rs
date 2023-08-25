use wasm_bindgen::prelude::*;
use percy_dom::prelude::*;
use web_sys;


#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

#[wasm_bindgen]
struct App {
  pdom: PercyDom
}

#[wasm_bindgen]
impl App {
    #[wasm_bindgen(constructor)]
    pub fn new () -> App {
        let start_view = html! { <div> Hello </div> };

        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();

        let mut pdom = PercyDom::new_append_to_mount(start_view, &body);

        let greetings = "Welcome to my app!";

        let title = "Pinnacle";

        let end_view = html! {
            // Use regular Rust comments within your html
            <main>
                <header>
                <h3 class=["title"]>{title}</h3>
                </header>

              /* Interpolate values using braces */
              <h1 class=[""]>{ greetings }</h1>

              <button
                class="giant-button"
                onclick=|_event| {
                   web_sys::console::log_1(&"Button Clicked!".into());
                }
              >
                // No need to wrap text in quotation marks (:
                Click me and check your console
              </button>
           </main>
        };

        pdom.update(end_view);

        App { pdom }
    }
}