use maplibre_gl_js::interface::{HtmlElement, MapOptions};
use yew::{Html, function_component, html, use_effect, use_node_ref, use_state};

#[function_component(App)]
fn app() -> Html {
    let container_ref = use_node_ref();
    let map_rendered = use_state(|| false);

    {
        let container_ref = container_ref.clone();
        use_effect(move || {
            if *map_rendered {
                return;
            }
            let container = container_ref
                .cast::<HtmlElement>()
                .expect("Container should be a valid HtmlElement");
            MapOptions::new(container)
                .with_style("https://demotiles.maplibre.org/style.json")
                .with_center([0., 0.])
                .with_zoom(1.0)
                .with_maplibre_logo()
                .build()
                .expect("Creating a map should work");
            map_rendered.set(true);
        });
    }

    html! { <div ref={container_ref}></div> }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    console_error_panic_hook::set_once();
    yew::Renderer::<App>::new().render();
}
