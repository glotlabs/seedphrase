use poly::page::wasm;
use poly::page::Page;
use poly_macro::impl_wasm_page;
use seedphrase_core::home_page;
use wasm_bindgen::prelude::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct HomePage(home_page::HomePage);

impl_wasm_page!(HomePage);

#[wasm_bindgen(js_name = "homePage")]
pub fn new(js_current_url: JsValue) -> Result<HomePage, JsValue> {
    let current_url = js_current_url
        .into_serde()
        .map_err(|err| format!("Failed to decode URL: {}", err))?;

    Ok(HomePage(home_page::HomePage { current_url }))
}
