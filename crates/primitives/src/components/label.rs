use leptos::{html, html::Label, prelude::*};
use web_sys::MouseEvent;

use crate::primitive::Primitive;

#[component]
pub fn LabelRoot(
  #[prop(optional, into)] for_html: MaybeProp<String>,

  #[prop(default=Callback::new(|_|{}), into)] on_mouse_down: Callback<MouseEvent>,

  #[prop(optional)] node_ref: NodeRef<Label>,
  children: ChildrenFn,

  #[prop(optional, into)] as_child: MaybeProp<bool>,
) -> impl IntoView {
  view! {
    <Primitive
      element={html::label}
      node_ref={node_ref}
      as_child={as_child}
      on:mousedown=move |ev: MouseEvent| {
        on_mouse_down.run(ev.clone());

        if ev.default_prevented() && ev.detail() > 1 {
          ev.prevent_default();
        }
      }
      {..}
      for=move || for_html.get()
    >
      {children()}
    </Primitive>
  }
}
