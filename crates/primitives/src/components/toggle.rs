use leptos::{
  html::{self, Button},
  prelude::*,
};
use web_sys::MouseEvent;

use crate::{
  primitive::Primitive,
  util::create_controllable_signal::{create_controllable_signal, CreateControllableSignalProps},
};

#[component]
pub fn ToggleRoot(
  #[prop(optional, into)] pressed: MaybeProp<bool>,
  #[prop(optional, into)] default_pressed: MaybeProp<bool>,
  #[prop(optional, into)] disabled: MaybeSignal<bool>,

  #[prop(default=Callback::new(|_|{}), into)] on_pressed_changed: Callback<bool>,
  #[prop(default=Callback::new(|_|{}), into)] on_click: Callback<MouseEvent>,

  #[prop(optional)] node_ref: NodeRef<Button>,
  children: ChildrenFn,

  #[prop(optional, into)] as_child: MaybeProp<bool>,
) -> impl IntoView {
  let (pressed, set_pressed) = create_controllable_signal(CreateControllableSignalProps {
    value: Signal::derive(move || pressed.get()),
    default_value: Signal::derive(move || default_pressed.get()),
    on_change: on_pressed_changed,
  });

  view! {
    <Primitive
      element={html::button}
      node_ref={node_ref}
      as_child={as_child}
      attr:data-state=move || {
        if pressed.get().unwrap_or_default() {
          "on"
        } else {
          "off"
        }
      }
      attr:data-disabled={disabled}
      {..}
      type="button"
      aria-pressed=move || pressed.get().unwrap_or_default().to_string()
      on:click=move |ev: MouseEvent| {
        on_click.run(ev.clone());

        if !disabled.get() {
          set_pressed.update(|pressed| *pressed = Some(!pressed.unwrap_or(false)));
        }
      }
    >
      {children()}
    </Primitive>
  }
}
