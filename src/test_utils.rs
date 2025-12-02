//! # Shared test functions

use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Document, HtmlScriptElement, js_sys};

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

const MAPLIBRE_SCRIPT_SRC: &str = "https://unpkg.com/maplibre-gl@^5.12.0/dist/maplibre-gl.js";
const MAPLIBRE_ID: &str = "maplibre-gl-e1b6fcf0-2a8c-46ae-9c7e-deeb271d32d7";

fn get_document() -> Document {
    let window = web_sys::window().expect("Should be able to get the window");
    window.document().expect("Document should be available")
}

fn load_maplibre_gl_inner(resolve: js_sys::Function, reject: js_sys::Function) {
    let on_error = Closure::wrap(Box::new(move || {
        reject
            .call0(&JsError::new(format!("Error loading {MAPLIBRE_ID}").as_str()).into())
            .expect("Calling reject should work");
    }) as Box<dyn FnMut()>);

    let insert_maplibre_if_not_there = Closure::wrap(Box::new(move || {
        // Check that we don't have maplibre already inserted
        if get_document().get_element_by_id(MAPLIBRE_ID).is_some() {
            resolve
                .call0(&JsValue::NULL)
                .expect("Calling resolve should work");
            return;
        }

        // Create script element
        let element: HtmlScriptElement = Into::<JsValue>::into(
            get_document()
                .create_element("script")
                .expect("Creating a script element should work"),
        )
        .into();
        element.set_id(MAPLIBRE_ID);
        element.set_src(MAPLIBRE_SCRIPT_SRC);
        element.set_async(true);
        element.set_onload(Some(&resolve));
        element.set_onerror(Some(on_error.as_ref().unchecked_ref()));

        // Insert
        get_document()
            .head()
            .expect("Head should be available")
            .append_child(&element)
            .expect("Appending script should work");
    }) as Box<dyn FnMut()>);

    if get_document().ready_state() == "loading" {
        get_document()
            .add_event_listener_with_callback(
                "DOMContentLoaded",
                insert_maplibre_if_not_there.as_ref().unchecked_ref(),
            )
            .expect("Adding listener to DOMContentLoaded should work");
    } else {
        insert_maplibre_if_not_there
            .as_ref()
            .unchecked_ref::<js_sys::Function>()
            .call0(&JsValue::NULL)
            .expect("Calling insert should work");
    }
}

/// Insert MapLibre GL into the document and ensure that they are available.
///
/// # Panics
///
/// This function is only intended to run on tests, errors aren't handled and
/// panics whenever anything goes wrong.
pub async fn load_maplibre_gl() {
    JsFuture::from(js_sys::Promise::new(&mut load_maplibre_gl_inner))
        .await
        .expect("Loading MapLibre gl should work");
}
