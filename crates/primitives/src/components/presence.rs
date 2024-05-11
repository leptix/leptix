use leptos::{
  ev::{animationcancel, animationend, animationstart},
  html::AnyElement,
  *,
};
use leptos_use::use_event_listener;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{js_sys::Object, AnimationEvent, CssStyleDeclaration};

use derive_more::{Deref, From};

use crate::util::create_state_machine::{create_state_machine, InvalidState, MachineState};

#[derive(Deref, From, Clone)]
struct StyleDeclaration(CssStyleDeclaration);

impl Default for StyleDeclaration {
  fn default() -> Self {
    Self(CssStyleDeclaration::from(JsValue::from(Object::new())))
  }
}

pub(crate) fn create_presence(
  is_present: Signal<bool>,
  node_ref: NodeRef<AnyElement>,
) -> Signal<bool> {
  let styles = StoredValue::<Option<StyleDeclaration>>::new(None);
  let prev_present = StoredValue::new(is_present.get());
  let prev_animation_name = StoredValue::new(String::from("none"));

  let initial = Signal::derive(move || {
    if is_present.get() {
      PresenceState::Mounted
    } else {
      PresenceState::Unmounted
    }
  });

  let (state, send) = create_state_machine(initial.into());

  Effect::new(move |_| {
    if let Some(node) = node_ref.get() {
      if let Ok(Some(computed_style)) = window().get_computed_style(&node) {
        styles.set_value(Some(computed_style.into()));
      }
    }
  });

  Effect::new(move |_| {
    let current_animation_name = styles
      .get_value()
      .unwrap_or_default()
      .get_property_value("animation-name")
      .unwrap_or("none".to_string());

    prev_animation_name.set_value(match state.get() {
      PresenceState::Mounted => current_animation_name,
      _ => "none".to_string(),
    });
  });

  Effect::new(move |_| {
    let was_present = prev_present.get_value();
    let has_present_changed = was_present != is_present.get();

    if !has_present_changed {
      return;
    }

    if styles.get_value().is_none() {
      styles.set_value(Some(StyleDeclaration::default()));
    }

    let styles = styles.get_value().unwrap_or_default();

    let current_animation_name = styles
      .get_property_value("animation-name")
      .unwrap_or("none".to_string());

    if is_present.get() {
      send.call(PresenceEvent::Mount);
    } else if current_animation_name == "none"
      || styles
        .get_property_value("display")
        .map(|display| display == "none")
        .unwrap_or(false)
    {
      send.call(PresenceEvent::Unmount);
    } else {
      let is_animating = prev_animation_name.get_value() != current_animation_name;

      if was_present && is_animating {
        send.call(PresenceEvent::AnimationOut);
      } else {
        send.call(PresenceEvent::Unmount);
      }
    }

    prev_present.set_value(is_present.get());
  });

  Effect::new(move |_| {
    let Some(node) = node_ref.get() else {
      send.call(PresenceEvent::AnimationEnd);
      return;
    };

    if node.is_null() {
      send.call(PresenceEvent::AnimationEnd);
      return;
    }

    let handle_start_node = node.clone();
    let remove_animation_start =
      use_event_listener(node_ref, animationstart, move |ev: AnimationEvent| {
        let Some(target) = ev.target() else {
          return;
        };

        let Some(target_el) = target.dyn_ref::<web_sys::Element>() else {
          return;
        };

        if target_el.eq(&handle_start_node) {
          if styles.get_value().is_none() {
            styles.set_value(Some(StyleDeclaration::default()));
          }

          prev_animation_name.set_value(
            styles
              .get_value()
              .unwrap_or_default()
              .get_property_value("animation-name")
              .unwrap_or("none".to_string()),
          );
        }
      });

    let handle_end_node = node.clone();
    let handle_animation_end = move |ev: AnimationEvent| {
      if styles.get_value().is_none() {
        styles.set_value(Some(StyleDeclaration::default()));
      }

      let current_animation_name = styles
        .get_value()
        .unwrap_or_default()
        .get_property_value("animation-name")
        .unwrap_or("none".to_string());

      let is_current_animation = current_animation_name.contains(&ev.animation_name());

      let Some(target) = ev.target() else {
        return;
      };

      let Some(target_el) = target.dyn_ref::<web_sys::Element>() else {
        return;
      };

      if target_el.eq(&handle_end_node) && is_current_animation {
        send.call(PresenceEvent::AnimationEnd);
      }
    };

    let remove_animation_end =
      use_event_listener(node_ref, animationend, handle_animation_end.clone());

    let remove_animation_cancel =
      use_event_listener(node_ref, animationcancel, handle_animation_end);

    on_cleanup(move || {
      remove_animation_start();
      remove_animation_end();
      remove_animation_cancel();
    });
  });

  Signal::derive(move || {
    state.get() == PresenceState::Mounted || state.get() == PresenceState::UnmountSuspended
  })
}

#[derive(Debug, Clone, PartialEq)]
enum PresenceState {
  Mounted,
  UnmountSuspended,
  Unmounted,
}

#[derive(Debug, Clone, Copy)]
enum PresenceEvent {
  Mount,
  Unmount,
  AnimationOut,
  AnimationEnd,
}

impl MachineState<Self, PresenceEvent> for PresenceState {
  fn send(&self, event: PresenceEvent) -> Result<Self, InvalidState> {
    match (self, event) {
      (Self::Mounted, PresenceEvent::Unmount) => Ok(Self::Unmounted),
      (Self::Mounted, PresenceEvent::AnimationOut) => Ok(Self::UnmountSuspended),
      (Self::Unmounted, PresenceEvent::Mount) => Ok(Self::Mounted),
      (Self::UnmountSuspended, PresenceEvent::AnimationEnd) => Ok(Self::Unmounted),
      (Self::UnmountSuspended, PresenceEvent::Mount) => Ok(Self::Mounted),
      _ => Err(InvalidState),
    }
  }
}
