use maplibre_gl_js::interface::MapOptions;
use web_sys::{HtmlCanvasElement, js_sys::Reflect, wasm_bindgen::JsValue};
use yew::{Html, function_component, html, use_effect, use_state};

fn is_webgl_supported() -> bool {
    let window = web_sys::window().expect("Window should be available");
    let document = window.document().expect("The document should be available");

    // Check if the class is directly missing
    if let Err(_) = Reflect::get(&window, &"WebGLRenderingContext".into()) {
        return false;
    }

    let canvas: HtmlCanvasElement = Into::<JsValue>::into(
        document
            .create_element("canvas")
            .expect("Creating an element should work"),
    )
    .into();

    // Obtain context
    let context = match (canvas.get_context("webgl2"), canvas.get_context("webgl")) {
        // If a JsException is raised, then it is supported but disabled
        (Err(_), _) => return false,
        (_, Err(_)) => return false,
        // No context available, then it is unsupported
        (Ok(None), Ok(None)) => return false,
        // Context is available
        (Ok(Some(webgl2_context)), _) => webgl2_context,
        (Ok(None), Ok(Some(webgl_context))) => webgl_context,
    };

    // Check if the context was properly initialized
    Reflect::get(&context, &"getParameter".into()).is_ok_and(|v| {
        v.js_typeof()
            .as_string()
            .expect("Result should be a string")
            == "function"
    })
}

#[function_component(App)]
fn app() -> Html {
    let webgl_supported = is_webgl_supported();
    let map_rendered = use_state(|| false);
    use_effect(move || {
        if *map_rendered {
            return;
        }
        if webgl_supported {
            MapOptions::new("map")
                .with_style("https://demotiles.maplibre.org/style.json")
                .with_center([-74.5, 40.])
                .with_zoom(2.)
                .build()
                .expect("Creating a map should work");
            map_rendered.set(true);
        }
    });
    html! { if webgl_supported { <div id="map"></div> } else { <p>{"WebGl not supported"}</p> } }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    console_error_panic_hook::set_once();
    yew::Renderer::<App>::new().render();
}
