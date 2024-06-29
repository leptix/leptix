use leptos::{html::AnyElement, *};

use crate::{primitive::Primitive, util::Attributes, Orientation};

#[component]
pub fn SeparatorRoot(
  #[prop(optional, into)] orientation: MaybeSignal<Orientation>,
  #[prop(optional, into)] decorative: MaybeSignal<bool>,

  #[prop(optional)] node_ref: NodeRef<AnyElement>,
  #[prop(attrs)] attrs: Attributes,
  #[prop(optional)] children: Option<ChildrenFn>,

  #[prop(optional, into)] as_child: MaybeProp<bool>,
) -> impl IntoView {
  let mut merged_attrs = if decorative.get_untracked() {
    vec![("role", "none".into_attribute())]
  } else {
    vec![
      (
        "aria-orientation",
        Signal::derive(move || orientation.get().to_string()).into_attribute(),
      ),
      ("role", "separator".into_attribute()),
    ]
  };

  merged_attrs.extend(attrs);
  merged_attrs.extend([(
    "data-orientation",
    Signal::derive(move || orientation.get().to_string()).into_attribute(),
  )]);

  let children = StoredValue::new(children);

  view! {
    <Primitive
      element=html::div
      attrs=merged_attrs
      node_ref=node_ref
      as_child=as_child
    >
      {children.with_value(|children| children.as_ref().map(|children| children()))}
    </Primitive>
  }
}
