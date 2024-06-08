use leptos::{html::AnyElement, *};

use crate::{components::primitive::Primitive, Attributes};

#[component]
pub fn AspectRatioRoot(
  #[prop(default=1.0f64.into(), into)] ratio: MaybeSignal<f64>,
  children: Children,
  #[prop(attrs)] attrs: Attributes,
  #[prop(optional)] node_ref: NodeRef<AnyElement>,
) -> impl IntoView {
  let mut merged_attrs = attrs.clone();

  merged_attrs.extend([(
    "style",
    "position: absolute; top: 0; right: 0; bottom: 0; left: 0".into_attribute(),
  )]);

  view! {
    <div
      style="position: relative; width: 100%;"
      style:padding-bottom=Signal::derive(move || format!("{}%", 100.0 / ratio.get()))
    >
      <Primitive
        element=html::div
        node_ref=node_ref
        attrs=merged_attrs
      >
        {children()}
      </Primitive>
    </div>
  }
}
