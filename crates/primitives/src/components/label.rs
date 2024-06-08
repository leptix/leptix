use leptos::{html::AnyElement, *};
use web_sys::MouseEvent;

use crate::{components::primitive::Primitive, Attributes};

#[component]
pub fn LabelRoot(
  #[prop(optional, into)] for_html: MaybeProp<String>,
  #[prop(default=(|_|{}).into(), into)] on_mouse_down: Callback<MouseEvent>,
  #[prop(optional)] node_ref: NodeRef<AnyElement>,
  #[prop(attrs)] attrs: Attributes,
  children: Children,
) -> impl IntoView {
  let mut merged_attrs = attrs.clone();
  merged_attrs.push(("for", for_html.into_attribute()));

  view! {
    <Primitive
      element=html::label
      attrs=merged_attrs
      node_ref=node_ref
      on:mousedown=move |ev: MouseEvent| {
          on_mouse_down.call(ev.clone());

        if ev.default_prevented() && ev.detail() > 1 {
          ev.prevent_default();
        }
      }
    >
      {children()}
    </Primitive>
  }
}
