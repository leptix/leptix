use floating_ui_leptos::{
  use_floating, ApplyState, AutoUpdateOptions, Hide, HideOptions, HideStrategy, IntoReference,
  LimitShift, LimitShiftOptions, MiddlewareVec, Offset, OffsetOptions, OffsetOptionsValues,
  Placement, Rect, Shift, ShiftOptions, Size, SizeOptions, Strategy, UseFloatingOptions,
  VirtualElementOrNodeRef,
};
use html::AnyElement;
use leptos::*;
use leptos_use::{use_element_size, UseElementSizeReturn};

use crate::{primitive::Primitive, util::Attributes};

use super::Side;

#[derive(Clone)]
pub enum Align {
  Start,
  Center,
  End,
}

#[derive(Clone)]
struct PopperContextValue {
  anchor: NodeRef<AnyElement>,
}

#[component]
pub fn Popper(children: ChildrenFn) -> impl IntoView {
  let anchor = NodeRef::<AnyElement>::new();

  provide_context(PopperContextValue { anchor });

  view! {
    <>{children()}</>
  }
}

#[component]
pub fn PopperAnchor(
  #[prop(optional, into)] virtual_ref: MaybeProp<NodeRef<AnyElement>>,

  #[prop(attrs)] attrs: Attributes,
  #[prop(optional)] node_ref: NodeRef<AnyElement>,
  children: ChildrenFn,

  #[prop(optional, into)] as_child: MaybeProp<bool>,
) -> impl IntoView {
  let PopperContextValue { anchor } =
    use_context().expect("PopperAnchor must be used in a Popper compoment");

  let condition = virtual_ref.clone();
  let has_no_virtual_ref = move || condition.get().is_none();

  Effect::new(move |_| {
    let Some(node) = node_ref.get() else {
      return;
    };

    anchor.load(
      &virtual_ref
        .get()
        .map(|virtual_node| virtual_node.get())
        .flatten()
        .unwrap_or(node),
    );
  });

  let children = StoredValue::new(children);

  view! {
    <Show when=has_no_virtual_ref>
      <Primitive
        attrs=attrs.clone()
        element=html::div
        node_ref=node_ref
        as_child=as_child
      >
        {children.with_value(|children| children())}
      </Primitive>
    </Show>
  }
}

#[derive(Clone, PartialEq)]
pub enum Sticky {
  Partial,
  Always,
}

#[derive(Clone, PartialEq)]
pub enum UpdatePositionStrategy {
  Optimized,
  Always,
}

#[derive(Clone)]
struct PopperContentContextValue {
  placed_side: Signal<Side>,
  on_arrow_change: Callback<NodeRef<AnyElement>>,
  arrow_x: Signal<Option<f64>>,
  arrow_y: Signal<Option<f64>>,
  should_hide_arrow: Signal<bool>,
}

#[component]
pub fn PopperContent(
  #[prop(default=Side::Bottom.into(), into)] side: MaybeSignal<Side>,
  #[prop(default=0.0.into(), into)] side_offset: MaybeSignal<f64>,
  #[prop(default=Align::Center.into(), into)] align: MaybeSignal<Align>,
  #[prop(default=0.0.into(), into)] align_offset: MaybeSignal<f64>,
  #[prop(default=0.0.into(), into)] arrow_padding: MaybeSignal<f64>,
  #[prop(default=true.into(), into)] avoid_collisions: MaybeSignal<bool>,
  #[prop(default=vec![].into(), into)] collision_boundary: MaybeSignal<Vec<NodeRef<AnyElement>>>,
  #[prop(default=0.0.into(), into)] collision_padding: MaybeSignal<f64>,
  #[prop(default=Sticky::Partial.into(), into)] sticky: MaybeSignal<Sticky>,
  #[prop(default=false.into(), into)] hide_when_detached: MaybeSignal<bool>,

  #[prop(default=UpdatePositionStrategy::Optimized.into(), into)]
  update_position_strategy: MaybeSignal<UpdatePositionStrategy>,

  #[prop(default=(|_|{}).into(), into)] on_placed: Callback<()>,

  #[prop(attrs)] attrs: Attributes,
  #[prop(optional)] node_ref: NodeRef<AnyElement>,
  children: ChildrenFn,

  #[prop(optional, into)] as_child: MaybeProp<bool>,
) -> impl IntoView {
  let PopperContextValue { anchor } =
    use_context().expect("PopperContent must be used in a Popper component");

  let arrow_ref = NodeRef::<AnyElement>::new();
  let UseElementSizeReturn {
    width: arrow_width,
    height: arrow_height,
  } = use_element_size(arrow_ref);

  let desired_placement = Signal::derive(move || {
    let align = align.get();

    match side.get() {
      Side::Top => match align {
        Align::Center => Placement::Top,
        Align::Start => Placement::TopStart,
        Align::End => Placement::TopEnd,
      },
      Side::Bottom => match align {
        Align::Center => Placement::Bottom,
        Align::Start => Placement::BottomStart,
        Align::End => Placement::BottomEnd,
      },
      Side::Left => match align {
        Align::Center => Placement::Left,
        Align::Start => Placement::LeftStart,
        Align::End => Placement::LeftEnd,
      },
      Side::Right => match align {
        Align::Center => Placement::Right,
        Align::Start => Placement::RightStart,
        Align::End => Placement::RightEnd,
      },
    }
  });

  let condition_boundary = collision_boundary.clone();
  let has_explicit_boundaries =
    Signal::derive(move || condition_boundary.with(|boundary| !boundary.is_empty()));

  let boundary = Signal::derive(move || {
    collision_boundary
      .get()
      .into_iter()
      .filter(|boundary| {
        boundary
          .get()
          .map(|boundary| boundary.is_null())
          .unwrap_or_default()
      })
      .collect::<Vec<_>>()
  });

  let options = UseFloatingOptions::default()
    .strategy(Strategy::Fixed.into())
    .while_elements_mounted_auto_update_with_options(MaybeSignal::derive(move || {
      AutoUpdateOptions::default()
        .animation_frame(update_position_strategy.get() == UpdatePositionStrategy::Always)
    }))
    .placement(desired_placement.into())
    .middleware(MaybeProp::derive(move || {
      let mut middleware: MiddlewareVec = vec![
        Box::new(Offset::new(OffsetOptions::Values(
          OffsetOptionsValues::default()
            .main_axis(side_offset.get())
            .alignment_axis(align_offset.get()),
        ))),
        Box::new(Size::new(SizeOptions::new().apply(
          &|ApplyState {
              state,
              available_width,
              available_height,
            }| {
            let Rect {
              width: anchor_width,
              height: anchor_height,
              ..
            } = state.rects.reference;
          },
        ))),
      ];

      if avoid_collisions.get() {
        let mut shift_options = ShiftOptions::default().main_axis(true).cross_axis(true);

        if let Sticky::Partial = sticky.get() {
          shift_options.limiter = Some(Box::new(LimitShift::new(LimitShiftOptions::default())));
        }

        middleware.push(Box::new(Shift::new(shift_options)));
      }

      if hide_when_detached.get() {
        middleware.push(Box::new(Hide::new(
          HideOptions::default().strategy(HideStrategy::ReferenceHidden),
        )));
      }

      Some(middleware)
    }));

  use_floating(
    anchor.into_reference(),
    NodeRef::<AnyElement>::new(),
    options,
  );

  view! {}
}
