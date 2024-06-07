use leptos::{html::AnyElement, *};
use web_sys::MouseEvent;

use crate::{components::primitive::Primitive, Attributes};

#[component]
pub fn LabelRoot(
  #[prop(optional)] for_html: Option<MaybeSignal<String>>,
  #[prop(default=Callback::new(|_:MouseEvent|{}), into)] on_mouse_down: Callback<MouseEvent>,
  #[prop(optional)] node_ref: NodeRef<AnyElement>,
  #[prop(attrs)] attrs: Attributes,
  children: Children,
) -> impl IntoView {
  let mut merged_attrs = attrs.clone();
  merged_attrs.push((
    "for",
    Signal::derive(move || for_html.as_ref().map(|for_html| for_html.get())).into_attribute(),
  ));

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
