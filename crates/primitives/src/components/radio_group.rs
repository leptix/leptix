use leptos::{
  ev::{keydown, keyup},
  html::AnyElement,
  *,
};
use leptos_use::{use_document, use_event_listener};
use wasm_bindgen::{closure::Closure, JsCast};
use web_sys::{FocusEvent, HtmlButtonElement, KeyboardEvent};

use crate::{
  components::{
    primitive::Primitive,
    radio::{Radio, RadioIndicator},
    roving_focus::{RovingFocusGroup, RovingFocusGroupItem},
  },
  util::{
    create_controllable_signal::{create_controllable_signal, CreateControllableSignalProps},
    Direction, Orientation,
  },
  Attributes,
};

#[derive(Clone)]
struct RadioGroupContextValue {
  name: MaybeProp<String>,
  required: Signal<bool>,
  disabled: Signal<bool>,
  value: Signal<Option<String>>,
  on_value_change: Callback<String>,
}

#[component]
pub fn RadioGroupRoot(
  #[prop(optional, into)] name: MaybeProp<String>,
  #[prop(optional, into)] required: MaybeSignal<bool>,
  #[prop(optional, into)] disabled: MaybeSignal<bool>,
  #[prop(optional, into)] should_loop: MaybeSignal<bool>,
  #[prop(optional, into)] value: MaybeProp<String>,
  #[prop(optional, into)] default_value: MaybeProp<String>,
  #[prop(optional, into)] orientation: MaybeSignal<Orientation>,
  #[prop(optional, into)] direction: MaybeSignal<Direction>,
  #[prop(default=(|_|{}).into(), into)] on_value_change: Callback<String>,
  #[prop(attrs)] attrs: Attributes,
  #[prop(optional)] node_ref: NodeRef<AnyElement>,
  children: Children,
) -> impl IntoView {
  let (value, set_value) = create_controllable_signal(CreateControllableSignalProps {
    value: Signal::derive(move || value.get()),
    default_value: Signal::derive(move || default_value.get()),
    on_change: on_value_change,
  });

  provide_context(RadioGroupContextValue {
    name,
    required: Signal::derive(move || required.get()),
    disabled: Signal::derive(move || disabled.get()),
    value: Signal::derive(move || value.get()),
    on_value_change: Callback::new(move |value| {
      set_value.set(value);
    }),
  });

  let mut merged_attrs = vec![
    ("role", "radiogroup".into_attribute()),
    ("aria-required", required.into_attribute()),
    (
      "aria-orientation",
      (move || orientation.get().to_string()).into_attribute(),
    ),
    ("data-disabled", disabled.into_attribute()),
    (
      "dir",
      (move || direction.get().to_string()).into_attribute(),
    ),
  ];

  merged_attrs.extend(attrs);

  view! {
    <RovingFocusGroup
      as_child=true
      orientation=Signal::derive(move || orientation.get())
      direction=Signal::derive(move || direction.get())
      should_loop=should_loop
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
pub fn RadioGroupItem(
  #[prop(into)] value: MaybeSignal<String>,
  #[prop(default=(|_|{}).into(), into)] on_focus: Callback<FocusEvent>,
  #[prop(default=(|_|{}).into(), into)] on_key_down: Callback<KeyboardEvent>,
  #[prop(optional, into)] disabled: MaybeSignal<bool>,
  #[prop(optional)] node_ref: NodeRef<AnyElement>,
  #[prop(attrs)] attrs: Attributes,
  children: Children,
) -> impl IntoView {
  let RadioGroupContextValue {
    disabled,
    value: context_value,
    required,
    name,
    on_value_change,
  } = use_context().expect("RadioGroupItem must be used in a RadioGroupRoot component");

  let is_disabled = Signal::derive(move || disabled.get() || disabled.get());

  let is_checked_value = value.clone();
  let is_checked = Signal::derive(move || context_value.get() == Some(is_checked_value.get()));
  let is_arrow_key_pressed = StoredValue::new(false);

  // _ = use_event_listener(use_document(), keydown, move |ev: KeyboardEvent| {
  //   if ["ArrowUp", "ArrowDown", "ArrowLeft", "ArrowRight"].contains(&ev.key().as_str()) {
  //     is_arrow_key_pressed.set_value(true);
  //   }
  // });

  // _ = use_event_listener(use_document(), keyup, move |ev: KeyboardEvent| {
  //   is_arrow_key_pressed.set_value(false);
  // });

  let on_check_value = value.clone();

  view! {
    <RovingFocusGroupItem
      as_child=true
      focusable=Signal::derive(move || !is_disabled.get())
      active=is_checked
    >
      <Radio
        value=value
        disabled=is_disabled
        required=required
        checked=is_checked
        name=name
        node_ref=node_ref
        attrs=attrs
        on_check=move |_| on_value_change.call(on_check_value.get())
        on:keydown=move |ev: KeyboardEvent| {
          on_key_down.call(ev.clone());

          if ev.key() == "Enter" {
            ev.prevent_default();
          }
        }
        on:focus=move |ev: FocusEvent| {
          on_focus.call(ev.clone());

          if is_arrow_key_pressed.get_value() {
            let Some(node) = node_ref.get() else {
              return;
            };

            let Some(node_el) = node.dyn_ref::<HtmlButtonElement>() else {
              return;
            };

            node_el.click();
          }
        }
      >
        {children()}
      </Radio>
    </RovingFocusGroupItem>
  }
}

#[component]
pub fn RadioGroupIndicator(
  #[prop(optional)] node_ref: NodeRef<AnyElement>,
  #[prop(attrs)] attrs: Attributes,
  // children: ChildrenFn,
) -> impl IntoView {
  view! {
    <RadioIndicator
      attrs=attrs
      node_ref=node_ref
    >
      {().into_view()}
    </RadioIndicator>
  }
}
