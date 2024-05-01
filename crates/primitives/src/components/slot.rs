use leptos::{html::ElementDescriptor, *};

use crate::Attributes;

#[slot]
pub struct Slot<El>
where
  El: ElementDescriptor + Clone + 'static,
{
  // #[prop(attrs)]
  attrs: Attributes,
  #[prop(optional_no_strip)]
  node_ref: Option<NodeRef<El>>,
  children: Children,
}

// pub(crate) fn merge_attrs(parent_attrs: Attributes, child_attrs: Attributes) -> Attributes {
pub(crate) fn merge_attrs(
  parent_attrs: Attributes,
  child_attrs: Vec<(String, Attribute)>,
) -> Vec<(String, Attribute)> {
  let mut cloned_parent_attrs = parent_attrs.clone();
  let mut child_attrs = child_attrs
    .into_iter()
    .map(|(name, child_attr)| {
      if let Some((_, parent_attr)) = parent_attrs.iter().find(|attr| attr.0 == name) {
        if let Some(position) = cloned_parent_attrs.iter().position(|item| item.0 == name) {
          cloned_parent_attrs.remove(position);
        }

        match name.as_str() {
          "class" => (name, merge_attr(parent_attr, &child_attr)),
          "style" => (name, merge_styles(parent_attr, &child_attr)),
          _ => (name, child_attr),
        }
      } else {
        (name, child_attr)
      }
    })
    .collect::<Vec<_>>();

  child_attrs.extend(
    cloned_parent_attrs
      .iter()
      .map(|(name, attr)| (name.to_string(), attr.clone())),
  );

  child_attrs
}

fn merge_attr(first: &Attribute, second: &Attribute) -> Attribute {
  if let Attribute::Fn(first) = first {
    return merge_attr(&first(), second);
  }

  if let Attribute::Fn(second) = second {
    return merge_attr(first, &second());
  }

  match (first, second) {
    (Attribute::String(first), Attribute::String(second)) => {
      format!("{} {}", first.trim(), second.trim()).into_attribute()
    }
    _ => second.clone(),
  }
}

fn merge_styles(first: &Attribute, second: &Attribute) -> Attribute {
  if let Attribute::Fn(first) = first {
    return merge_styles(&first(), second);
  }

  if let Attribute::Fn(second) = second {
    return merge_styles(first, &second());
  }

  match (first, second) {
    (Attribute::String(first), Attribute::String(second)) => {
      format!("{}; {}", first.trim(), second.trim()).into_attribute()
    }
    _ => second.clone(),
  }
}

#[test]
fn merged_attrs() {
  assert_eq!(
    merge_attrs(
      vec![(
        "class",
        Attribute::String("text-blue-500 outline-none".into())
      )],
      vec![(
        "class".to_string(),
        Attribute::Fn(std::rc::Rc::new(|| "bg-neutral-500".into_attribute())) // Attribute::String("bg-neutral-500".into())
      )]
    ),
    vec![(
      "class".to_string(),
      Attribute::String("text-blue-500 outline-none bg-neutral-500".into())
    )]
  )
}
