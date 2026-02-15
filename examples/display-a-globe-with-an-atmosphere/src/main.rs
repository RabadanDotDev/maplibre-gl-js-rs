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
                "projection": {
                    "type": "globe"
                },
                "sources": {
                    "satellite": {
                        "tiles": ["https://tiles.maps.eox.at/wmts/1.0.0/s2cloudless-2020_3857/default/g/{z}/{y}/{x}.jpg"],
                        "type": "raster"
                    },
                },
                "layers": [
                    {
                        "id": "Satellite",
                        "type": "raster",
                        "source": "satellite",
                    },
                ],
                "sky": {
                    "atmosphere-blend": [
                        "interpolate",
                        ["linear"],
                        ["zoom"],
                        0, 1,
                        5, 1,
                        7, 0
                    ]
                },
                "light": {
                    "anchor": "map",
                    "position": [1.5, 90, 80]
                }
            }))
            .with_center([137.9150899566626, 36.25956997955441])
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
