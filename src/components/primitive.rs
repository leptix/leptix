use leptos::{
  html::{AnyElement, ElementDescriptor},
  *,
};

use crate::Attributes;

use super::slot::merge_attrs;

#[component]
pub fn Primitive<El: ElementDescriptor + 'static>(
  element: fn() -> HtmlElement<El>,
  children: Children,

  #[prop(attrs)] attrs: Attributes,
  #[prop(optional_no_strip)] as_child: Option<bool>,
  #[prop(optional_no_strip)] node_ref: Option<NodeRef<AnyElement>>,
) -> impl IntoView {
  // logging::log!("primitive attrs: {attrs:?}");

  if as_child.unwrap_or(false) {
    map_items_to_children(children().as_children(), attrs, node_ref)
  } else {
    let el = element()
      .attrs(attrs)
      .child(children().into_view())
      .into_any();

    if let Some(node_ref) = node_ref {
      el.node_ref(node_ref)
    } else {
      el
    }
    .into_view()
  }
}

fn map_items_to_children(
  children: &[View],
  attrs: Attributes,
  node_ref: Option<NodeRef<AnyElement>>,
) -> View {
  if children.len() == 0 {
    None::<bool>.into_view()
  } else {
    children
      .into_iter()
      .map(|child| match child {
        View::Element(el) => {
          let el = el.clone().into_html_element();

          let child_attrs = el
            .get_attribute_names()
            .iter()
            .filter_map(|name| {
              let name = name.as_string()?;
              Some((name.clone(), el.get_attribute(&name)?.into_attribute()))
            })
            .collect::<Vec<_>>();

          let merged_attrs = merge_attrs(attrs.clone(), child_attrs);

          for (name, attr) in merged_attrs {
            if let Some(attr) = attr.as_nameless_value_string() {
              _ = el.set_attribute(&name, &attr);
            }
          }

          if let Some(node_ref) = node_ref {
            el.node_ref(node_ref)
          } else {
            el
          }
          .into_view()
        }
        View::Component(comp) => map_items_to_children(&comp.children, attrs.clone(), node_ref),
        _ => child.into_view(),
      })
      .collect_view()
  }
}
