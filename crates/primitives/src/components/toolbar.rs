use leptos::{html::AnyElement, *};
use wasm_bindgen::JsCast;
use web_sys::{HtmlAnchorElement, KeyboardEvent};

use crate::{
  components::{
    primitive::Primitive,
    roving_focus::{RovingFocusGroup, RovingFocusGroupItem},
    separator::SeparatorRoot,
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
  #[prop(optional, into)] orientation: MaybeSignal<Orientation>,
  #[prop(optional, into)] direction: MaybeSignal<Direction>,
  #[prop(default=true.into(), into)] should_loop: MaybeSignal<bool>,
  #[prop(attrs)] attrs: Attributes,
  #[prop(optional)] node_ref: NodeRef<AnyElement>,
  children: Children,
) -> impl IntoView {
  provide_context(ToolbarContextValue {
    orientation: Signal::derive(move || orientation.get()),
    direction: Signal::derive(move || direction.get()),
  });

  let mut merged_attrs = vec![
    ("role", "toolbar".into_attribute()),
    (
      "aria-orientation",
      (move || orientation.get().to_string()).into_attribute(),
    ),
    (
      "dir",
      (move || direction.get().to_string()).into_attribute(),
    ),
  ];

  merged_attrs.extend(attrs);

  view! {
    <RovingFocusGroup
      orientation=Signal::derive(move || orientation.get())
      direction=Signal::derive(move || direction.get())
      should_loop=Signal::derive(move || should_loop.get())
    >
      <Primitive
        element=html::div
        attrs=merged_attrs
        node_ref=node_ref
      >
        {children()}
      </Primitive>
    </RovingFocusGroup>
  }
}

#[component]
pub fn ToolbarSeparator(
  #[prop(optional)] node_ref: NodeRef<AnyElement>,
  #[prop(attrs)] attrs: Attributes,
) -> impl IntoView {
  let ToolbarContextValue { orientation, .. } =
    use_context().expect("ToolbarSeparator must be used in a ToolbarRoot component");

  let orientation = Signal::derive(move || match orientation.get() {
    Orientation::Horizontal => Orientation::Vertical,
    Orientation::Vertical => Orientation::Horizontal,
  });

  view! {
    <SeparatorRoot
      orientation=orientation
      attrs=attrs
      node_ref=node_ref
    />
  }
}

#[component]
pub fn ToolbarButton(
  #[prop(optional)] as_child: Option<bool>,
  #[prop(optional, into)] disabled: MaybeSignal<bool>,
  #[prop(attrs)] attrs: Attributes,
  #[prop(optional)] node_ref: NodeRef<AnyElement>,
  children: Children,
) -> impl IntoView {
  let mut merged_attrs = vec![("type", "button".into_attribute())];
  merged_attrs.extend(attrs);

  view! {
    <RovingFocusGroupItem
      as_child=true
      focusable=Signal::derive(move || disabled.get())
    >
      <Primitive
        element=html::button
        as_child=as_child
        attrs=merged_attrs
        node_ref=node_ref
      >
        {children()}
      </Primitive>
    </RovingFocusGroupItem>
  }
}

#[component]
pub fn ToolbarLink(
  #[prop(default=(|_|{}).into(), into)] on_key_down: Callback<KeyboardEvent>,
  #[prop(attrs)] attrs: Attributes,
  #[prop(optional)] node_ref: NodeRef<AnyElement>,
  children: Children,
) -> impl IntoView {
  view! {
    <RovingFocusGroupItem
      as_child=true
      focusable=true
    >
      <Primitive
        element=html::a
        attrs=attrs
        node_ref=node_ref
        on:keydown=move |ev: KeyboardEvent| {
            on_key_down.call(ev.clone());

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

  #[prop(optional, into)] disabled: MaybeSignal<bool>,
  #[prop(optional, into)] orientation: MaybeSignal<Orientation>,
  #[prop(optional, into)] direction: MaybeSignal<Direction>,

  #[prop(attrs)] attrs: Attributes,
  #[prop(optional)] node_ref: NodeRef<AnyElement>,
  children: ChildrenFn,
) -> impl IntoView {
  let context = use_context::<ToolbarContextValue>()
    .expect("ToolbarToggleGroup must be in a ToolbarRoot component");

  let mut merged_attrs = vec![
    (
      "data-orientation",
      (move || context.orientation.get().to_string()).into_attribute(),
    ),
    (
      "dir",
      (move || context.direction.get().to_string()).into_attribute(),
    ),
  ];
  merged_attrs.extend(attrs);

  view! {
    <ToggleGroupRoot
      kind=kind
      attrs=merged_attrs
      disabled=Signal::derive(move || disabled.get())
      orientation=Signal::derive(move || orientation.get())
      direction=Signal::derive(move || direction.get())
      roving_focus=false
      node_ref=node_ref
    >
      {children()}
    </ToggleGroupRoot>
  }
}

#[component]
pub fn ToolbarToggleItem(
  #[prop(optional, into)] disabled: MaybeSignal<bool>,
  #[prop(into)] value: MaybeSignal<String>,

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
        disabled=Signal::derive(move || disabled.get())
        value=value
        node_ref=node_ref
      >
        {children()}
      </ToggleGroupItem>
    </ToolbarButton>
  }
}
