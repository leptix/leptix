use leptos::{html::AnyElement, *};

use crate::{components::primitive::Primitive, util::Orientation, Attributes};

#[component]
pub fn Separator(
  #[prop(optional)] orientation: Option<MaybeSignal<Orientation>>,
  #[prop(optional)] decorative: Option<MaybeSignal<bool>>,
  #[prop(attrs)] attrs: Attributes,
  #[prop(optional)] node_ref: NodeRef<AnyElement>,
) -> impl IntoView {
  let mut merged_attrs = if decorative
    .map(|decorative| decorative.get())
    .unwrap_or(false)
  {
    vec![("role", "none".into_attribute())]
  } else {
    vec![
      (
        "aria-orientation",
        Signal::derive(move || {
          orientation
            .map(|orientation| orientation.get())
            .unwrap_or_default()
            .to_string()
        })
        .into_attribute(),
      ),
      ("role", "separator".into_attribute()),
    ]
  };

  merged_attrs.extend(attrs.into_iter());
  merged_attrs.extend([(
    "data-orientation",
    Signal::derive(move || {
      orientation
        .map(|orientation| orientation.get())
        .unwrap_or_default()
        .to_string()
    })
    .into_attribute(),
  )]);

  view! {
    <Primitive
      element=html::div
      attrs=merged_attrs
      node_ref=node_ref
    >
      {().into_view()}
    </Primitive>
  }
}
