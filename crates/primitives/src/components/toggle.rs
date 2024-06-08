use leptos::{html::AnyElement, *};
use web_sys::MouseEvent;

use crate::{
  components::primitive::Primitive,
  util::create_controllable_signal::{create_controllable_signal, CreateControllableSignalProps},
  Attributes,
};

#[component]
pub fn ToggleRoot(
  #[prop(optional, into)] pressed: Option<MaybeSignal<bool>>,
  #[prop(optional, into)] default_pressed: Option<MaybeSignal<bool>>,
  #[prop(optional, into)] disabled: MaybeSignal<bool>,
  #[prop(default=(|_|{}).into(), into)] on_pressed_changed: Callback<bool>,
  #[prop(default=(|_|{}).into(), into)] on_click: Callback<MouseEvent>,

  #[prop(optional)] node_ref: NodeRef<AnyElement>,
  #[prop(attrs)] attrs: Attributes,
  children: Children,
) -> impl IntoView {
  let (pressed, set_pressed) = create_controllable_signal(CreateControllableSignalProps {
    value: Signal::derive(move || pressed.map(|pressed| pressed.get())),
    default_value: Signal::derive(move || {
      default_pressed.map(|default_pressed| default_pressed.get())
    }),
    on_change: on_pressed_changed,
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
    ("data-disabled", disabled.into_attribute()),
  ];

  merged_attrs.extend(attrs);

  view! {
    <Primitive
      attrs=merged_attrs
      element=html::button
      node_ref=node_ref
      on:click=move |ev: MouseEvent| {
          on_click.call(ev.clone());

        if !disabled.get() {
          set_pressed.update(|pressed| *pressed = Some(!pressed.unwrap_or(false)));
        }
      }
    >
      {children()}
    </Primitive>
  }
}
