use leptos::{html::AnyElement, *};
use web_sys::MouseEvent;

use crate::{components::primitive::Primitive, Attributes};

#[component]
pub fn LabelRoot(
  #[prop(attrs)] attrs: Attributes,
  children: Children,
  #[prop(optional)] for_html: Option<Signal<String>>,
  #[prop(optional)] on_mouse_down: Option<Callback<MouseEvent>>,
  #[prop(optional)] node_ref: NodeRef<AnyElement>,
) -> impl IntoView {
  let mut merged_attrs = attrs.clone();
  merged_attrs.push((
    "for",
    Signal::derive(move || for_html.map(|for_html| for_html.get())).into_attribute(),
  ));

  view! {
    <Primitive
      element=html::label
      attrs=merged_attrs
      node_ref=Some(node_ref)
      on:mousedown=move |ev: MouseEvent| {
        if let Some(on_mouse_down) = on_mouse_down {
          on_mouse_down(ev.clone());
        }

        if ev.default_prevented() && ev.detail() > 1 {
          ev.prevent_default();
        }
      }
    >
      {children()}
    </Primitive>
  }
}
