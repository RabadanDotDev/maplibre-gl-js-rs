use maplibre_gl_js::interface::MapOptions;
use yew::{Html, function_component, html, use_effect};

#[function_component(App)]
fn app() -> Html {
    use_effect(move || {
        MapOptions::new("map")
            .with_style("https://demotiles.maplibre.org/style.json")
            .with_center([74.5, 40.])
            .with_zoom(3.)
            .without_interactivity()
            .build()
            .expect("Creating a map should work");
    });
    html! { <div id="map"></div> }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    console_error_panic_hook::set_once();
    yew::Renderer::<App>::new().render();
}
