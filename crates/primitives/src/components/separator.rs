use leptos::{
  html::{self, Div},
  prelude::*,
};

use crate::{primitive::Primitive, Orientation};

#[component]
pub fn SeparatorRoot(
  #[prop(optional, into)] orientation: MaybeSignal<Orientation>,
  #[prop(optional, into)] decorative: MaybeSignal<bool>,

  #[prop(optional)] node_ref: NodeRef<Div>,
  #[prop(optional)] children: Option<ChildrenFn>,

  #[prop(optional, into)] as_child: MaybeProp<bool>,
) -> impl IntoView {
  let children = StoredValue::new(children);

  view! {
    <Primitive
      element={html::div}
      node_ref={node_ref}
      as_child={as_child}
      attr:data-orientation=move || orientation.get().to_string()
      {..}
      role=move || if decorative.get() { "none" } else { "separator" }
      aria-orientation=move || (!decorative.get()).then_some(orientation.get().to_string())
    >
      {children.with_value(|children| children.as_ref().map(|children| children()))}
    </Primitive>
  }
}
