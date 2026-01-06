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
            .with_style(serde_json::json!({
                "version": 8,
                "sources": {
                    "raster-tiles": {
                        "type": "raster",
                        "tiles": ["https://tile.openstreetmap.org/{z}/{x}/{y}.png"],
                        "tileSize": 256,
                        "minzoom": 0,
                        "maxzoom": 19
                    }
                },
                "layers": [
                    {
                        "id": "simple-tiles",
                        "type": "raster",
                        "source": "raster-tiles",
                        "attribution": "© OpenStreetMap contributors",
                    }
                ],
                "id": "blank"
            }))
            .with_center([0., 0.])
            .with_zoom(0.)
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
