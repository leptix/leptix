use leptos::{html::AnyElement, *};
use wasm_bindgen::JsCast;
use web_sys::{HtmlAnchorElement, KeyboardEvent};

use crate::{
  components::{
    primitive::Primitive,
    roving_focus::{RovingFocusGroup, RovingFocusGroupItem},
    separator::Separator,
    toggle_group::{ToggleGroupItem, ToggleGroupRoot},
  },
  util::{Direction, Orientation},
  Attributes,
};

use super::toggle_group::ToggleGroupKind;

#[derive(Clone)]
struct ToolbarContextValue {
  orientation: Signal<Orientation>,
  direction: Signal<Direction>,
}

#[component]
pub fn ToolbarRoot(
  #[prop(optional)] orientation: Option<Signal<Orientation>>,
  #[prop(optional)] direction: Option<Signal<Direction>>,
  #[prop(optional)] should_loop: Option<Signal<bool>>,
  #[prop(attrs)] attrs: Attributes,
  #[prop(optional)] node_ref: NodeRef<AnyElement>,
  children: Children,
) -> impl IntoView {
  provide_context(ToolbarContextValue {
    orientation: Signal::derive(move || {
      orientation
        .map(|orientation| orientation.get())
        .unwrap_or_default()
    }),
    direction: Signal::derive(move || {
      direction
        .map(|direction| direction.get())
        .unwrap_or_default()
    }),
  });

  let mut merged_attrs = vec![
    ("role", "toolbar".into_attribute()),
    (
      "aria-orientation",
      Signal::derive(move || {
        match orientation
          .map(|orientation| orientation.get())
          .unwrap_or_default()
        {
          Orientation::Horizontal => "horizontal",
          Orientation::Vertical => "vertical",
        }
      })
      .into_attribute(),
    ),
    (
      "dir",
      Signal::derive(move || {
        match direction
          .map(|direction| direction.get())
          .unwrap_or_default()
        {
          Direction::LeftToRight => "ltr",
          Direction::RightToLeft => "rtl",
        }
      })
      .into_attribute(),
    ),
  ];

  merged_attrs.extend(attrs.into_iter());

  view! {
    <RovingFocusGroup
      orientation=Signal::derive(move || orientation.map(|orientation| orientation.get()).unwrap_or_default())
      direction=Signal::derive(move || direction.map(|direction| direction.get()).unwrap_or_default())
      should_loop=Signal::derive(move || should_loop.map(|should_loop| should_loop.get()).unwrap_or(true))
    >
      <Primitive
        element=html::div
        attrs=merged_attrs
        node_ref=Some(node_ref)
      >
        {children()}
      </Primitive>
    </RovingFocusGroup>
  }
}

#[component]
pub fn ToolbarSeparator(
  #[prop(optional)] orientation: Option<Signal<Orientation>>,
  #[prop(optional)] decorative: Option<Signal<bool>>,
  #[prop(optional)] node_ref: NodeRef<AnyElement>,
  #[prop(attrs)] attrs: Attributes,
) -> impl IntoView {
  let context = use_context::<ToolbarContextValue>()
    .expect("ToolbarSeparator must be used in a ToolbarRoot component");

  let orientation = Signal::derive(move || match context.orientation.get() {
    Orientation::Horizontal => Orientation::Vertical,
    Orientation::Vertical => Orientation::Horizontal,
  });

  view! {
    <Separator
      orientation=orientation
      attrs=attrs
      node_ref=node_ref
    />
  }
}

#[component]
pub fn ToolbarButton(
  #[prop(optional)] as_child: Option<bool>,
  #[prop(optional)] disabled: Option<Signal<bool>>,
  #[prop(attrs)] attrs: Attributes,
  #[prop(optional)] node_ref: NodeRef<AnyElement>,
  children: Children,
) -> impl IntoView {
  let mut merged_attrs = vec![("type", "button".into_attribute())];
  merged_attrs.extend(attrs.into_iter());

  view! {
    <RovingFocusGroupItem
      as_child=true
      focusable=Signal::derive(move || disabled.map(|disabled| disabled.get()).unwrap_or(false))
    >
      <Primitive
        element=html::button
        as_child=as_child
        attrs=merged_attrs
        node_ref=Some(node_ref)
      >
        {children()}
      </Primitive>
    </RovingFocusGroupItem>
  }
}

#[component]
pub fn ToolbarLink(
  #[prop(optional)] on_key_down: Option<Callback<KeyboardEvent>>,
  #[prop(attrs)] attrs: Attributes,
  #[prop(optional)] node_ref: NodeRef<AnyElement>,
  children: Children,
) -> impl IntoView {
  view! {
    <RovingFocusGroupItem
      as_child=true
      focusable=Signal::derive(move || true)
    >
      <Primitive
        element=html::a
        attrs=attrs
        node_ref=Some(node_ref)
        on:keydown=move |ev: KeyboardEvent| {
          if let Some(on_key_down) = on_key_down {
            on_key_down(ev.clone());
          }

          if ev.key() == " " {
            if let Some(current_target) = ev.current_target() {
              if let Some(current_target) = current_target.dyn_ref::<HtmlAnchorElement>() {
                current_target.click();
              }
            }
          }
        }
      >
        {children()}
      </Primitive>
    </RovingFocusGroupItem>
  }
}

#[component]
pub fn ToolbarToggleGroup(
  kind: ToggleGroupKind,

  #[prop(optional)] disabled: Option<Signal<bool>>,
  #[prop(optional)] orientation: Option<Signal<Orientation>>,
  #[prop(optional)] direction: Option<Signal<Direction>>,

  #[prop(attrs)] attrs: Attributes,
  #[prop(optional)] node_ref: NodeRef<AnyElement>,
  children: ChildrenFn,
) -> impl IntoView {
  let context = use_context::<ToolbarContextValue>()
    .expect("ToolbarToggleGroup must be in a ToolbarRoot component");

  let mut merged_attrs = vec![
    (
      "data-orientation",
      Signal::derive(move || match context.orientation.get() {
        Orientation::Horizontal => "horizontal",
        Orientation::Vertical => "vertical",
      })
      .into_attribute(),
    ),
    (
      "dir",
      Signal::derive(move || match context.direction.get() {
        Direction::LeftToRight => "ltr",
        Direction::RightToLeft => "rtl",
      })
      .into_attribute(),
    ),
  ];
  merged_attrs.extend(attrs.into_iter());

  view! {
    <ToggleGroupRoot
      kind=kind
      attrs=merged_attrs
      disabled=Signal::derive(move || disabled.map(|disabled| disabled.get()).unwrap_or(false))
      orientation=Signal::derive(move || orientation.map(|orientation| orientation.get()).unwrap_or_default())
      direction=Signal::derive(move || direction.map(|direction| direction.get()).unwrap_or_default())
      roving_focus=Signal::derive(|| false)
    >
      {children()}
    </ToggleGroupRoot>
  }
}

#[component]
pub fn ToolbarToggleItem(
  #[prop(optional)] disabled: Option<Signal<bool>>,
  value: Signal<String>,

  #[prop(attrs)] attrs: Attributes,
  #[prop(optional)] node_ref: NodeRef<AnyElement>,
  children: ChildrenFn,
) -> impl IntoView {
  view! {
    <ToolbarButton
      as_child=true
    >
      <ToggleGroupItem
        attrs=attrs
        disabled=Signal::derive(move || disabled.map(|disabled| disabled.get()).unwrap_or(false))
        value=value
        node_ref=node_ref
      >
        {children()}
      </ToggleGroupItem>
    </ToolbarButton>
  }
}
