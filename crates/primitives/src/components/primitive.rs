use leptos::{
  html::{AnyElement, ElementDescriptor},
  *,
};

use crate::Attributes;

#[component]
pub fn Primitive<El: ElementDescriptor + 'static>(
  element: fn() -> HtmlElement<El>,

  #[prop(optional)] node_ref: NodeRef<AnyElement>,
  #[prop(attrs)] attrs: Attributes,
  children: Children,

  #[prop(optional, into)] as_child: MaybeProp<bool>,
) -> impl IntoView {
  view! {
    {move || {
      if as_child.get().unwrap_or_default() {
        map_items_to_children(children().as_children(), attrs, node_ref)
      } else {
        element()
          .attrs(attrs)
          .child(children().into_view())
          .into_any()
          .node_ref(node_ref)
          .into_view()
      }
    }}
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
