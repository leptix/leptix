use html::AnyElement;
pub use itertools::Either;
use leptos::*;

use crate::util::Attributes;

use super::Side;

pub enum Align {
  Start,
  Center,
  End,
}

#[derive(Clone)]
struct PopperContextValue {
  anchor: Option<NodeRef<AnyElement>>,
  on_anchor_change: Callback<NodeRef<AnyElement>>,
}

pub enum Sticky {
  Partial,
  Always,
}

pub enum UpdatePositionStrategy {
  Optimized,
  Always,
}

#[component]
pub fn PopperContent(
  #[prop(optional, into)] side: MaybeProp<Side>,
  #[prop(optional, into)] side_offset: MaybeProp<f64>,
  #[prop(optional, into)] align: MaybeProp<Align>,
  #[prop(optional, into)] align_offset: MaybeProp<f64>,
  #[prop(optional, into)] arrow_padding: MaybeProp<f64>,
  #[prop(optional, into)] avoid_collisions: MaybeProp<bool>,
  #[prop(optional, into)] collision_boundary: MaybeProp<
    Either<NodeRef<AnyElement>, Vec<NodeRef<AnyElement>>>,
  >,
  #[prop(optional, into)] collision_padding: MaybeProp<f64>,
  #[prop(optional, into)] sticky: MaybeProp<Sticky>,
  #[prop(optional, into)] hide_when_detached: MaybeProp<bool>,
  #[prop(optional, into)] update_position_strategy: MaybeProp<UpdatePositionStrategy>,

  #[prop(default=(|_|{}).into(), into)] on_placed: Callback<()>,

  #[prop(attrs)] attrs: Attributes,
  #[prop(optional)] node_ref: NodeRef<AnyElement>,
  children: ChildrenFn,

  #[prop(optional, into)] as_child: MaybeProp<bool>,
) -> impl IntoView {
  view! {}
}
