use html::AnyElement;
use leptos::*;

use crate::util::Attributes;

use super::Side;

#[component]
pub fn PopperContent(
  #[prop(optional, into)] side: MaybeProp<Side>,

  #[prop(attrs)] attrs: Attributes,
  #[prop(optional)] node_ref: NodeRef<AnyElement>,
  children: ChildrenFn,

  #[prop(optional, into)] as_child: MaybeProp<bool>,
) -> impl IntoView {
  view! {}
}
