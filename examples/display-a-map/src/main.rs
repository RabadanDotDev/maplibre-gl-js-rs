use maplibre_gl_js::interface::MapOptions;
use yew::{Html, function_component, html, use_effect, use_state};

#[function_component(App)]
fn app() -> Html {
    let map_rendered = use_state(|| false);
    use_effect(move || {
        if *map_rendered {
            return;
        }
        MapOptions::new("map")
            .with_style("https://demotiles.maplibre.org/style.json")
            .with_center([0., 0.])
            .with_zoom(1.0)
            .with_maplibre_logo()
            .build()
            .expect("Creating a map should work");
        map_rendered.set(true);
    });
    html! { <div id="map"></div> }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    console_error_panic_hook::set_once();
    yew::Renderer::<App>::new().render();
}
