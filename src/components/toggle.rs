use leptos::{html::AnyElement, *};
use web_sys::MouseEvent;

use crate::{
  components::primitive::Primitive,
  util::create_controllable_signal::{create_controllable_signal, CreateControllableSignalProps},
  Attributes,
};

#[component]
pub fn ToggleRoot(
  #[prop(optional)] pressed: Option<MaybeSignal<bool>>,
  #[prop(optional)] disabled: Option<MaybeSignal<bool>>,
  #[prop(optional)] default_pressed: Option<MaybeSignal<bool>>,
  #[prop(optional)] on_pressed_changed: Option<Callback<bool>>,
  #[prop(optional)] on_click: Option<Callback<MouseEvent>>,
  #[prop(optional)] node_ref: NodeRef<AnyElement>,

  #[prop(attrs)] attrs: Attributes,
  children: Children,
) -> impl IntoView {
  let (pressed, set_pressed) = create_controllable_signal(CreateControllableSignalProps {
    value: Signal::derive(move || pressed.map(|pressed| pressed.get())),
    default_value: Signal::derive(move || {
      default_pressed.map(|default_pressed| default_pressed.get())
    }),
    on_change: Callback::new(move |value| {
      if let Some(on_pressed_changed) = on_pressed_changed {
        on_pressed_changed(value);
      }
    }),
  });

  let mut merged_attrs = vec![
    ("type", "button".into_attribute()),
    (
      "aria-pressed",
      Signal::derive(move || pressed.get().unwrap_or(false).to_string()).into_attribute(),
    ),
    (
      "data-state",
      Signal::derive(move || {
        if pressed.get().unwrap_or(false) {
          "on"
        } else {
          "off"
        }
      })
      .into_attribute(),
    ),
    (
      "data-disabled",
      Signal::derive(move || disabled.map(|disabled| disabled.get()).unwrap_or(false))
        .into_attribute(),
    ),
  ];

  merged_attrs.extend(attrs.into_iter());

  view! {
    <Primitive
      attrs=merged_attrs
      element=html::button
      node_ref=node_ref
      on:click=move |ev: MouseEvent| {
        if let Some(on_click) = on_click {
          on_click(ev.clone());
        }

        if disabled.map(|disabled| disabled.get()).unwrap_or(false) == false {
          set_pressed.update(|pressed| *pressed = Some(!pressed.unwrap_or(false)));
        }
      }
    >
      {children()}
    </Primitive>
  }
}
