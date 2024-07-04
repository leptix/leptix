use ev::{focusin, keydown, CustomEvent, Event, FocusEvent, KeyboardEvent, PointerEvent};
use html::AnyElement;
use itertools::Either;
use leptos::*;
use leptos_use::{
  use_document, use_event_listener, use_event_listener_with_options, use_timeout_fn,
  UseEventListenerOptions, UseTimeoutFnReturn,
};
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::{AddEventListenerOptions, CustomEventInit, Document, Node};

use crate::{primitive::Primitive, util::Attributes};

#[derive(Clone)]
pub struct DismissableLayerContext {
  layers: RwSignal<Vec<NodeRef<AnyElement>>>,
  layers_with_outside_pointer_events_disabled: RwSignal<Vec<NodeRef<AnyElement>>>,
  branches: RwSignal<Vec<NodeRef<AnyElement>>>,
}

pub type PointerDownOutsideEvent = CustomEvent;
pub type FocusOutsideEvent = CustomEvent;

#[component]
pub fn DismissableLayer(
  #[prop(optional, into)] disable_outside_pointer_events: MaybeProp<bool>,

  #[prop(default=(|_|{}).into(), into)] on_escape_key_down: Callback<KeyboardEvent>,
  #[prop(default=(|_|{}).into(), into)] on_pointer_down_outside: Callback<PointerDownOutsideEvent>,
  #[prop(default=(|_|{}).into(), into)] on_focus_outside: Callback<FocusOutsideEvent>, // PointerDownOutsideEvent
  #[prop(default=(|_|{}).into(), into)] on_interact_outside: Callback<
    Either<PointerDownOutsideEvent, FocusOutsideEvent>,
  >, // FocusOutsideEvent
  #[prop(default=(|_|{}).into(), into)] on_dismiss: Callback<()>,

  #[prop(attrs)] attrs: Attributes,
  #[prop(optional)] node_ref: NodeRef<AnyElement>,
  children: ChildrenFn,

  #[prop(optional, into)] as_child: MaybeProp<bool>,
) -> impl IntoView {
  let DismissableLayerContext {
    layers,
    layers_with_outside_pointer_events_disabled,
    branches,
  } = use_context::<DismissableLayerContext>()
    .expect("DismissableLayer must be used in a component that provides DismissableLayerContext");

  let owner_document = Signal::derive(move || {
    if let Some(node) = node_ref.get() {
      node.owner_document().unwrap_or(document())
    } else {
      document()
    }
  });

  let index = Signal::derive(move || {
    layers.with(|layers| {
      layers.iter().position(|item| {
        let Some(item_node) = item.get() else {
          return false;
        };

        let Some(node) = node_ref.get() else {
          return false;
        };

        item_node.eq(&node)
      })
    })
  });

  let highest_layer_with_outside_pointer_events_disabled = Signal::derive(move || {
    layers_with_outside_pointer_events_disabled.with(|layers| layers.last().cloned())
  });

  let highest_layer_with_outside_pointer_events_disabled_index = Signal::derive(move || {
    let layer = highest_layer_with_outside_pointer_events_disabled
      .get()
      .and_then(|layer| layer.get())?;

    layers.with(|layers| {
      layers.iter().position(|item| {
        let Some(item_node) = item.get() else {
          return false;
        };

        item_node.eq(&layer)
      })
    })
  });

  let is_body_pointer_events_disabled = Signal::derive(move || {
    layers_with_outside_pointer_events_disabled.with(|layers| !layers.is_empty())
  });
  let is_pointer_events_enabled = Signal::derive(move || {
    let (Some(index), Some(highest_layer_index)) = (
      index.get(),
      highest_layer_with_outside_pointer_events_disabled_index.get(),
    ) else {
      return false;
    };

    index >= highest_layer_index
  });

  let on_pointer_down_capture = use_pointer_down_outside(
    Callback::new(move |ev: PointerDownOutsideEvent| {
      let Some(target) = ev.target() else {
        return;
      };

      let is_pointer_down_on_branch = branches.with(|branches| {
        branches.iter().any(|branch| {
          branch
            .get()
            .map(|branch| branch.contains(target.dyn_ref::<Node>()))
            .unwrap_or_default()
        })
      });

      if !is_pointer_events_enabled.get() || is_pointer_down_on_branch {
        return;
      }

      on_pointer_down_outside.call(ev.clone());
      on_interact_outside.call(Either::Left(ev.clone()));

      if !ev.default_prevented() {
        on_dismiss.call(());
      }
    }),
    owner_document,
  );

  let (on_focus_capture, on_blur_capture) = use_focus_outside(
    Callback::new(move |ev: FocusOutsideEvent| {
      let Some(target) = ev.target() else {
        return;
      };

      let is_focus_in_branch = branches.with(|branches| {
        branches.iter().any(|branch| {
          let Some(branch) = branch.get() else {
            return false;
          };

          let Some(target_el) = target.dyn_ref::<web_sys::HtmlElement>() else {
            return false;
          };

          *branch == *target_el
        })
      });

      if is_focus_in_branch {
        return;
      }

      on_focus_outside.call(ev.clone());
      on_interact_outside.call(Either::Right(ev.clone()));

      if !ev.default_prevented() {
        on_dismiss.call(());
      }
    }),
    owner_document,
  );

  use_event_listener_with_options(
    owner_document,
    keydown,
    move |ev: KeyboardEvent| {
      let is_highest_layer = index.get() == Some(layers.with(|layers| layers.len() - 1));
      if !is_highest_layer {
        return;
      }

      on_escape_key_down.call(ev.clone());

      if !ev.default_prevented() {
        ev.prevent_default();
        on_dismiss.call(());
      }
    },
    UseEventListenerOptions::default().capture(true),
  );

  Effect::new(move |_| {
    let Some(node) = node_ref.get() else {
      return;
    };

    if disable_outside_pointer_events.get().unwrap_or_default() {
      if layers_with_outside_pointer_events_disabled.with(|layers| layers.len() == 0) {
        // owner_document.get().body().map(|body| body.style)
      }

      layers_with_outside_pointer_events_disabled.update(|layers| {
        if !layers
          .iter()
          .any(|layer| layer.get().map(|layer| *layer == *node).unwrap_or_default())
        {
          layers.push(node_ref);
        }
      });
    }

    layers.update(|layers| {
      if !layers
        .iter()
        .any(|layer| layer.get().map(|layer| *layer == *node).unwrap_or_default())
      {
        layers.push(node_ref);
      }
    });

    dispatch_update();

    on_cleanup(move || {
      if disable_outside_pointer_events.get().unwrap_or_default()
        && layers_with_outside_pointer_events_disabled.with(|layers| layers.len() == 1)
      {}
    });
  });

  on_cleanup(move || {
    let Some(node) = node_ref.get() else {
      return;
    };

    if node.is_null() {
      return;
    }

    layers.update(|layers| {
      let Some(position) = layers
        .iter()
        .position(|layer| layer.get().map(|layer| *layer == *node).unwrap_or_default())
      else {
        return;
      };

      layers.remove(position);
    });

    layers_with_outside_pointer_events_disabled.update(|layers| {
      let Some(position) = layers
        .iter()
        .position(|layer| layer.get().map(|layer| *layer == *node).unwrap_or_default())
      else {
        return;
      };

      layers.remove(position);
    });

    dispatch_update();
  });

  use_event_listener(
    use_document(),
    ev::Custom::new("dismissable_layer.update"),
    move |ev: CustomEvent| {},
  );

  Effect::new(move |_| {
    let Some(node) = node_ref.get() else {
      return;
    };

    _ = node.style("pointer-events", move || {
      is_body_pointer_events_disabled
        .get()
        .then_some(if is_pointer_events_enabled.get() {
          "auto"
        } else {
          "none"
        })
    });
  });

  view! {
    <Primitive
      attrs=attrs
      element=html::div

      on:focus=move |_| {
        on_focus_capture();
      }
      on:blur=move|_| {
        on_blur_capture();
      }
      on:pointerdown=move|_| {
        on_pointer_down_capture();
      }

      node_ref=node_ref
      as_child=as_child
    >
      {children()}
    </Primitive>
  }
}

#[component]
pub fn DismissableLayerBranch(
  #[prop(attrs)] attrs: Attributes,
  #[prop(optional)] node_ref: NodeRef<AnyElement>,
  children: ChildrenFn,

  #[prop(optional, into)] as_child: MaybeProp<bool>,
) -> impl IntoView {
  let DismissableLayerContext { branches, .. } = use_context().expect(
    "DismissableLayerBranch must be used in a component that provides DismissableLayerContext",
  );

  Effect::new(move |_| {
    let Some(node) = node_ref.get() else {
      return;
    };

    branches.update(|branches| {
      if !branches.iter().any(|branch| {
        branch
          .get()
          .map(|branch| *branch == *node)
          .unwrap_or_default()
      }) {
        branches.push(node_ref);
      }
    });

    on_cleanup(move || {
      branches.update(|branches| {
        if let Some(position) = branches.iter().position(|branch| {
          branch
            .get()
            .map(|branch| *branch == *node)
            .unwrap_or_default()
        }) {
          branches.remove(position);
        };
      });
    });
  });

  view! {
    <Primitive
      attrs=attrs
      element=html::div
      node_ref=node_ref
      as_child=as_child
    >
      {children()}
    </Primitive>
  }
}

fn use_pointer_down_outside(
  on_pointer_down_outside: Callback<PointerDownOutsideEvent>,
  owner_document: Signal<Document>,
) -> impl Fn() {
  let is_pointer_inside_tree = StoredValue::new(false);
  let handle_click = StoredValue::new(Callback::new(move |_| {}));

  Effect::new(move |_| {
    let handle_pointer_down = Closure::<dyn Fn(_)>::new(move |ev: PointerEvent| {
      let handle_ev = ev.clone();
      let handle_event = move || {
        handle_and_dispatch_custom_event(
          "dismissable_layer.pointer_down_outside",
          Some(on_pointer_down_outside),
          &handle_ev,
        )
      };

      if ev.pointer_type() == "touch" {
        let handle_and_dispatch_pointer_down_outside_event =
          Closure::<dyn Fn()>::new(handle_event.clone());

        // _ = owner_document.get().remove_event_listener_with_callback(
        //   "click",
        //   ,
        // );

        handle_and_dispatch_pointer_down_outside_event.forget();

        // handle_click.set_value(Closure::<dyn Fn()>::new(handle_event.clone()));
      } else {
        handle_event();
      }
    });

    let UseTimeoutFnReturn { start, .. } = use_timeout_fn(
      move |_: ()| {
        // _ = owner_document.get().add_event_listener_with_callback(
        //   "pointerdown",
        //   handle_pointer_down.as_ref().unchecked_ref(),
        // );
      },
      0.0,
    );

    start(());

    on_cleanup(move || {
      _ = owner_document.get().remove_event_listener_with_callback(
        "pointerdown",
        handle_pointer_down.as_ref().unchecked_ref(),
      );
    });
  });

  move || is_pointer_inside_tree.set_value(false)
}

fn use_focus_outside(
  on_pointer_down_outside: Callback<FocusOutsideEvent>,
  owner_document: Signal<Document>,
) -> (impl Fn(), impl Fn()) {
  let is_focus_inside_tree = StoredValue::new(false);

  Effect::new(move |_| {
    use_event_listener(owner_document.get(), focusin, move |ev| {
      let Some(target) = ev.target() else {
        return;
      };

      if target.is_null() || is_focus_inside_tree.get_value() {
        return;
      }

      handle_and_dispatch_custom_event(
        "dismissable_layer.focus_outside",
        Some(on_pointer_down_outside),
        &ev,
      );
    });
  });

  (
    move || is_focus_inside_tree.set_value(true),
    move || is_focus_inside_tree.set_value(false),
  )
}

fn dispatch_update() {
  let Ok(event) = CustomEvent::new("dismissable_layer.update") else {
    return;
  };

  _ = document().dispatch_event(&event);
}

fn handle_and_dispatch_custom_event(
  name: &'static str,
  handler: Option<Callback<CustomEvent>>,
  detail: &Event,
) {
  let Some(target) = detail.target() else {
    return;
  };
  let mut event_desc = CustomEventInit::new();
  event_desc.bubbles(false).cancelable(true).detail(detail);

  let Ok(event) = CustomEvent::new_with_event_init_dict(name, &event_desc) else {
    return;
  };

  if let Some(handler) = handler {
    let mut event_listener_desc = AddEventListenerOptions::new();
    event_listener_desc.once(true);

    let event_handler = Closure::<dyn FnMut(_)>::new(move |ev: CustomEvent| {
      handler.call(ev);
    });

    _ = target.add_event_listener_with_callback_and_add_event_listener_options(
      name,
      event_handler.as_ref().unchecked_ref(),
      &event_listener_desc,
    );

    event_handler.forget();
  }

  _ = target.dispatch_event(&event);
}
