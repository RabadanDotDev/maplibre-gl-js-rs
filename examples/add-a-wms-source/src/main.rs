use maplibre_gl_js::interface::MapOptions;
use yew::{Html, function_component, html, use_effect};

#[function_component(App)]
fn app() -> Html {
    use_effect(move || {
        MapOptions::new("map")
            .with_style(serde_json::json!({
                "version": 8,
                "sources": {
                    "wms-test-source": {
                        "type": "raster",
                        // use the tiles option to specify a WMS tile source URL
                        // https://maplibre.org/maplibre-style-spec/sources/
                        "tiles": [
                            "https://ows.terrestris.de/osm/service?service=WMS&request=GetMap&version=1.1.1&layers=TOPO-WMS%2COSM-Overlay-WMS&styles=&format=image%2Fpng&transparent=true&info_format=text%2Fhtml&tiled=false&srs=EPSG:3857&bbox={bbox-epsg-3857}&width=256&height=256"
                        ],
                        "tileSize": 256
                    }
                },
                "layers": [{
                    "id": "wms-test-layer",
                    "type": "raster",
                    "source": "wms-test-source",
                    "paint": {}
                }]
            }))
            .with_center([-74.5447, 40.6892])
            .with_zoom(8.)
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
