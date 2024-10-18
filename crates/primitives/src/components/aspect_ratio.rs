use leptos::{html, html::Div, prelude::*};

use crate::primitive::Primitive;

#[component]
pub fn AspectRatioRoot(
  #[prop(default=1.0f64.into(), into)] ratio: MaybeSignal<f64>,

  #[prop(optional)] node_ref: NodeRef<Div>,
  children: ChildrenFn,

  #[prop(optional, into)] as_child: MaybeProp<bool>,
) -> impl IntoView {
  view! {
    <div
      style="position: relative; width: 100%;"
      style:padding-bottom=Signal::derive(move || format!("{}%", 100.0 / ratio.get()))
    >
      <Primitive
        element={html::div}
        node_ref={node_ref}
        as_child={as_child}
        {..}
        style="position: absolute; top: 0; right: 0; bottom: 0; left: 0"
      >
        {children()}
      </Primitive>
    </div>
  }
}
