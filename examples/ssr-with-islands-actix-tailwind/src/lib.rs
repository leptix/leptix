pub mod app;
mod primitives;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
  use app::*;
  use leptos::*;

  console_error_panic_hook::set_once();

  leptos::leptos_dom::HydrationCtx::stop_hydrating();
}
