#![recursion_limit = "1024"]
pub mod app;
pub mod pages;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use app::*;
    use leptos::prelude::*;

    console_error_panic_hook::set_once();

    hydrate_islands();
}
