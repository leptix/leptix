use leptos::{html::AnyElement, *};
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
  name: Signal<Option<String>>,
  required: Signal<bool>,
  disabled: Signal<bool>,
  value: Signal<Option<String>>,
  on_value_change: Callback<String>,
}

#[component]
pub fn RadioGroupRoot(
  #[prop(optional)] name: Option<Signal<String>>,
  #[prop(optional)] required: Option<Signal<bool>>,
  #[prop(optional)] disabled: Option<Signal<bool>>,
  #[prop(optional)] should_loop: Option<Signal<bool>>,
  #[prop(optional)] value: Option<Signal<String>>,
  #[prop(optional)] default_value: Option<Signal<String>>,
  #[prop(optional)] orientation: Option<Signal<Orientation>>,
  #[prop(optional)] direction: Option<Signal<Direction>>,
  #[prop(optional)] on_value_change: Option<Callback<String>>,
  #[prop(attrs)] attrs: Attributes,
  #[prop(optional)] node_ref: NodeRef<AnyElement>,
  children: Children,
) -> impl IntoView {
  let (value, set_value) = create_controllable_signal(CreateControllableSignalProps {
    value: Signal::derive(move || value.map(|value| value.get())),
    default_value: Signal::derive(move || default_value.map(|default_value| default_value.get())),
    on_change: Callback::new(move |value| {
      if let Some(on_value_change) = on_value_change {
        on_value_change(value);
      }
    }),
  });

  provide_context(RadioGroupContextValue {
    name: Signal::derive(move || name.map(|name| name.get())),
    required: Signal::derive(move || required.map(|required| required.get()).unwrap_or(false)),
    disabled: Signal::derive(move || disabled.map(|disabled| disabled.get()).unwrap_or(false)),
    value: Signal::derive(move || value.get()),
    on_value_change: Callback::new(move |value| {
      set_value.set(value);
    }),
  });

  let mut merged_attrs = vec![
    ("role", "radiogroup".into_attribute()),
    (
      "aria-required",
      Signal::derive(move || required.map(|required| required.get()).unwrap_or(false))
        .into_attribute(),
    ),
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
      "data-disabled",
      Signal::derive(move || disabled.map(|disabled| disabled.get()).unwrap_or(false))
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
      as_child=true
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
pub fn RadioGroupItem(
  value: Signal<String>,
  #[prop(optional)] on_focus: Option<Callback<FocusEvent>>,
  #[prop(optional)] on_key_down: Option<Callback<KeyboardEvent>>,
  #[prop(optional)] disabled: Option<Signal<bool>>,
  #[prop(optional)] node_ref: NodeRef<AnyElement>,
  #[prop(attrs)] attrs: Attributes,
  children: Children,
) -> impl IntoView {
  let context = use_context::<RadioGroupContextValue>()
    .expect("RadioGroupItem must be used in a RadioGroupRoot component");

  let is_disabled = Signal::derive(move || {
    context.disabled.get() || disabled.map(|disabled| disabled.get()).unwrap_or(false)
  });

  let is_checked = Signal::derive(move || context.value.get() == Some(value.get()));
  let is_arrow_key_pressed = StoredValue::new(false);

  Effect::new(move |_| {
    let handle_key_down = Closure::<dyn Fn(_)>::new(move |ev: KeyboardEvent| {
      if ["ArrowUp", "ArrowDown", "ArrowLeft", "ArrowRight"].contains(&ev.key().as_str()) {
        is_arrow_key_pressed.set_value(true);
      }
    });

    let handle_key_up = Closure::<dyn Fn()>::new(move || {
      is_arrow_key_pressed.set_value(false);
    });

    _ = document()
      .add_event_listener_with_callback("keydown", handle_key_down.as_ref().unchecked_ref());
    _ =
      document().add_event_listener_with_callback("keyup", handle_key_up.as_ref().unchecked_ref());

    on_cleanup(move || {
      _ = document()
        .remove_event_listener_with_callback("keydown", handle_key_down.as_ref().unchecked_ref());
      _ = document()
        .remove_event_listener_with_callback("keyup", handle_key_up.as_ref().unchecked_ref());

      handle_key_down.forget();
      handle_key_up.forget();
    });
  });

  let node_ref = NodeRef::<AnyElement>::new();

  view! {
    <RovingFocusGroupItem
      as_child=true
      focusable=Signal::derive(move || !is_disabled.get())
      active=Signal::derive(move || is_checked.get())
    >
      <Radio
        value=Signal::derive(move || value.get())
        disabled=Signal::derive(move || is_disabled.get())
        required=Signal::derive(move || context.required.get())
        checked=Signal::derive(move || is_checked.get())
        name=Signal::derive(move || context.name.get())
        node_ref=node_ref
        attrs=attrs
        on_check=Callback::new(move |_| (context.on_value_change)(value.get()))
        on:keydown=move |ev: KeyboardEvent| {
          if let Some(on_key_down) = on_key_down {
            on_key_down(ev.clone());
          }

          if ev.key() == "Enter" {
            ev.prevent_default();
          }
        }
        on:focus=move |ev: FocusEvent| {
          if let Some(on_focus) = on_focus {
            on_focus(ev.clone());
          }

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
