use leptos::{
  html::{AnyElement, ElementDescriptor},
  *,
};

use crate::util::Attributes;

#[component]
pub fn Primitive<El: ElementDescriptor + 'static>(
  element: fn() -> HtmlElement<El>,

  #[prop(optional)] node_ref: NodeRef<AnyElement>,
  #[prop(attrs)] attrs: Attributes,
  children: ChildrenFn,

  #[prop(optional, into)] as_child: MaybeProp<bool>,
) -> impl IntoView {
  let children = StoredValue::new(children);
  let attrs = StoredValue::new(attrs);

  view! {
    <Show
      when=move || as_child.get().unwrap_or_default()
      fallback=move || element()
        .attrs(attrs.get_value())
        .child(children.with_value(|children| children()).into_view())
        .into_any()
        .node_ref(node_ref)
        .into_view()
    >
      {
        map_items_to_children(
          children.with_value(|children| children()).as_children(),
          attrs.get_value(),
          node_ref,
        )
      }
    </Show>
  }
}

fn map_items_to_children(
  children: &[View],
  attrs: Attributes,
  node_ref: NodeRef<AnyElement>,
) -> View {
  if children.is_empty() {
    None::<bool>.into_view()
  } else {
    children
      .iter()
      .map(|child| match child {
        View::Element(el) => el
          .clone()
          .into_html_element()
          .node_ref(node_ref)
          .attrs(attrs.clone())
          .into_view(),
        View::Component(comp) => map_items_to_children(&comp.children, attrs.clone(), node_ref),
        _ => child.into_view(),
      })
      .collect_view()
  }
}
