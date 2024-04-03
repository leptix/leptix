use leptos::{html::AnyElement, *};
use wasm_bindgen::{closure::Closure, JsCast, JsValue};
use web_sys::{js_sys::Object, AnimationEvent, CssStyleDeclaration};

use crate::util::create_state_machine::{create_state_machine, InvalidState, MachineState};

pub(crate) struct CreatePresenceResult {
  pub(crate) is_present: Signal<bool>,
  pub(crate) node_ref: NodeRef<AnyElement>,
}

pub(crate) fn create_presence(is_present: Signal<bool>) -> CreatePresenceResult {
  let styles = StoredValue::new(CssStyleDeclaration::from(JsValue::from(Object::new())));
  let prev_present = StoredValue::new(is_present.get());
  let prev_animation_name = StoredValue::new(String::from("none"));
  let node_ref = NodeRef::<AnyElement>::new();

  let initial = Signal::derive(move || {
    if is_present.get() {
      PresenceState::Mounted
    } else {
      PresenceState::Unmounted
    }
  });

  let (state, send) = create_state_machine(initial.into());

  node_ref.on_load(move |el| {
    if let Ok(Some(computed_style)) = window().get_computed_style(&el) {
      styles.set_value(computed_style);
    }
  });

  Effect::new(move |_| {
    let current_animation_name = styles
      .get_value()
      .get_property_value("animation-name")
      .unwrap_or("none".to_string());

    prev_animation_name.set_value(match state.get() {
      PresenceState::Mounted => current_animation_name,
      _ => "none".to_string(),
    });
  });

  Effect::new(move |_| {
    let was_present = prev_present.get_value();
    let has_present_changed = was_present == is_present.get();

    if has_present_changed == false {
      return;
    }

    let current_animation_name = styles
      .get_value()
      .get_property_value("animation-name")
      .unwrap_or("none".to_string());

    logging::log!("checking present states");
    if is_present.get() {
      logging::log!("is_present");
      send(PresenceEvent::Mount);
    } else if current_animation_name == "none"
      || styles
        .get_value()
        .get_property_value("display")
        .map(|display| display == "none")
        .unwrap_or(false)
    {
      logging::log!("display was none");
      send(PresenceEvent::Unmount);
    } else {
      logging::log!("checking if animating");
      let is_animating = prev_animation_name.get_value() != current_animation_name;

      logging::log!("check if present and animating");
      if was_present && is_animating {
        logging::log!("anim out");
        send(PresenceEvent::AnimationOut);
        logging::log!("anim out done");
      } else {
        logging::log!("unm");
        send(PresenceEvent::Unmount);
        logging::log!("unm done");
      }
      logging::log!("done checking present states");
    }
    logging::log!("setting prev present");

    prev_present.set_value(is_present.get());
    logging::log!("prev present set");
  });

  Effect::new(move |_| {
    let Some(node) = node_ref.get() else {
      send(PresenceEvent::AnimationEnd);
      return;
    };

    if node.is_null() {
      send(PresenceEvent::AnimationEnd);
      return;
    }

    let handle_end_node = node.clone();
    let handle_animation_end = Closure::<dyn FnMut(_)>::new(move |ev: AnimationEvent| {
      let current_animation_name = styles
        .get_value()
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
        send(PresenceEvent::AnimationEnd);
      }
    });

    let handle_start_node = node.clone();
    let handle_animation_start = Closure::<dyn FnMut(_)>::new(move |ev: AnimationEvent| {
      let Some(target) = ev.target() else {
        return;
      };

      let Some(target_el) = target.dyn_ref::<web_sys::Element>() else {
        return;
      };

      if target_el.eq(&handle_start_node) {
        prev_animation_name.set_value(
          styles
            .get_value()
            .get_property_value("animation-name")
            .unwrap_or("none".to_string()),
        );
      }
    });

    _ = node.add_event_listener_with_callback(
      "animationstart",
      handle_animation_start.as_ref().unchecked_ref(),
    );
    _ = node.add_event_listener_with_callback(
      "animationend",
      handle_animation_end.as_ref().unchecked_ref(),
    );
    _ = node.add_event_listener_with_callback(
      "animationcancel",
      handle_animation_end.as_ref().unchecked_ref(),
    );

    on_cleanup(move || {
      _ = node.remove_event_listener_with_callback(
        "animationstart",
        handle_animation_start.as_ref().unchecked_ref(),
      );
      _ = node.remove_event_listener_with_callback(
        "animationend",
        handle_animation_end.as_ref().unchecked_ref(),
      );
      _ = node.remove_event_listener_with_callback(
        "animationcancel",
        handle_animation_end.as_ref().unchecked_ref(),
      );

      handle_animation_end.forget();
      handle_animation_start.forget();
    });
  });

  CreatePresenceResult {
    is_present: Signal::derive(move || {
      state.get() == PresenceState::Mounted || state.get() == PresenceState::UnmountSuspended
    }),
    node_ref,
  }
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
    let foo = match (self, event) {
      (Self::Mounted, PresenceEvent::Unmount) => Ok(Self::Unmounted),
      (Self::Mounted, PresenceEvent::AnimationOut) => Ok(Self::UnmountSuspended),
      (Self::Unmounted, PresenceEvent::Mount) => Ok(Self::Mounted),
      (Self::UnmountSuspended, PresenceEvent::AnimationEnd) => Ok(Self::Unmounted),
      (Self::UnmountSuspended, PresenceEvent::Mount) => Ok(Self::Mounted),
      _ => return Err(InvalidState),
    };

    logging::log!("{foo:?}");

    foo
  }
}
