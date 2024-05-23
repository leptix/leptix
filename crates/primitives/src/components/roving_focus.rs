use std::collections::HashMap;

use leptos::{ev::EventDescriptor, html::AnyElement, *};
use leptos_use::use_event_listener;
use wasm_bindgen::{closure::Closure, JsCast};
use web_sys::{Event, FocusEvent, KeyboardEvent, MouseEvent};

use itertools::Either;

use crate::{
  components::{
    collection::{use_collection_context, CollectionContextValue},
    primitive::Primitive,
  },
  util::{
    create_controllable_signal::{create_controllable_signal, CreateControllableSignalProps},
    create_id::create_id,
    Direction, Orientation,
  },
  Attributes,
};

use super::collection::create_collection_item_ref;

#[derive(Clone, PartialEq, Debug, Eq)]
struct ItemData {
  id: String,
  focusable: Signal<bool>,
  active: Signal<bool>,
}

impl Ord for ItemData {
  fn cmp(&self, other: &Self) -> std::cmp::Ordering {
    self.id.cmp(&other.id)
  }
}

impl PartialOrd for ItemData {
  fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    Some(self.cmp(other))
  }
}

#[derive(Clone)]
struct RovingContextValue {
  orientation: Signal<Option<Orientation>>,
  direction: Signal<Option<Direction>>,
  should_loop: Signal<Option<bool>>,
  current_tab_stop_id: Signal<Option<String>>,
  on_item_focus: Callback<String>,
  on_item_shift_tab: Callback<()>,
  on_focusable_item_add: Callback<()>,
  on_focusable_item_remove: Callback<()>,
}

#[derive(Clone)]
struct OnEntryFocus;

impl EventDescriptor for OnEntryFocus {
  const BUBBLES: bool = false;
  type EventType = web_sys::Event;

  fn name(&self) -> Oco<'static, str> {
    "roving_focus_group.on_entry_focus".into()
  }

  fn event_delegation_key(&self) -> Oco<'static, str> {
    "$$$roving_focus_group.on_entry_focus".into()
  }
}

#[component]
pub(crate) fn RovingFocusGroup(
  #[prop(optional)] as_child: Option<bool>,
  #[prop(optional)] orientation: Option<MaybeSignal<Orientation>>,
  #[prop(optional)] direction: Option<MaybeSignal<Direction>>,
  #[prop(optional)] should_loop: Option<MaybeSignal<bool>>,
  #[prop(optional)] current_tab_stop_id: Option<MaybeSignal<String>>,
  #[prop(optional)] default_current_tab_stop_id: Option<MaybeSignal<String>>,
  #[prop(optional)] on_current_tab_stop_id_change: Option<Callback<Option<String>>>,
  #[prop(optional)] on_entry_focus: Option<Callback<Event>>,
  #[prop(optional)] on_mouse_down: Option<Callback<MouseEvent>>,
  #[prop(optional)] on_focus: Option<Callback<FocusEvent>>,
  #[prop(optional)] on_blur: Option<Callback<FocusEvent>>,
  #[prop(optional)] prevent_scroll_on_entry_focus: Option<bool>,
  #[prop(attrs)] attrs: Attributes,
  children: Children,
) -> impl IntoView {
  provide_context(CollectionContextValue::<ItemData, _> {
    collection_ref: NodeRef::<html::AnyElement>::new(),
    item_map: RwSignal::new(HashMap::new()),
  });

  let node_ref = NodeRef::<html::Div>::new();

  let value = Signal::derive(move || current_tab_stop_id.as_ref().map(|id| id.get()));
  let default_value =
    Signal::derive(move || default_current_tab_stop_id.as_ref().map(|id| id.get()));

  let (current_tab_stop_id, set_current_tab_stop_id) =
    create_controllable_signal(CreateControllableSignalProps {
      value,
      default_value,
      on_change: Callback::new(move |value| {
        if let Some(on_current_tab_stop_id_change) = on_current_tab_stop_id_change {
          on_current_tab_stop_id_change.call(Some(value))
        }
      }),
    });

  let (is_tabbing_back_out, set_is_tabbing_back_out) = create_signal(false);

  let CollectionContextValue { collection_ref, .. } =
    use_context::<CollectionContextValue<ItemData, AnyElement>>().expect("what happen");

  let get_items = use_collection_context::<ItemData, html::AnyElement>();
  let is_click_focus = StoredValue::new(false);

  let (focusable_items_count, set_focusable_items_count) = create_signal(0);

  _ = use_event_listener(node_ref, OnEntryFocus, move |ev: web_sys::Event| {
    if let Some(on_entry_focus) = on_entry_focus {
      on_entry_focus.call(ev);
    }
  });

  provide_context(RovingContextValue {
    orientation: Signal::derive(move || orientation.as_ref().map(|orientation| orientation.get())),
    direction: Signal::derive(move || direction.as_ref().map(|direction| direction.get())),
    should_loop: Signal::derive(move || should_loop.as_ref().map(|should_loop| should_loop.get())),
    current_tab_stop_id: Signal::derive(move || current_tab_stop_id.get()),
    on_item_focus: Callback::new(move |item| {
      set_current_tab_stop_id.set(item);
    }),
    on_item_shift_tab: Callback::new(move |_| {
      set_is_tabbing_back_out.set(true);
    }),
    on_focusable_item_add: Callback::new(move |_| {
      set_focusable_items_count.update(|count| {
        *count += 1;
      });
    }),
    on_focusable_item_remove: Callback::new(move |_| {
      set_focusable_items_count.update(|count| {
        *count -= 1;
      });
    }),
  });

  let mut merged_attrs = vec![
    // (
    //   "tabindex",
    //   Signal::derive(move || {
    //     if is_tabbing_back_out.get() || focusable_items_count.get() == 0 {
    //       -1
    //     } else {
    //       0
    //     }
    //   })
    //   .into_attribute(),
    // ),
    (
      "data-orientation",
      (move || orientation.map(|orientation| orientation.get().to_string())).into_attribute(),
    ),
  ];

  merged_attrs.extend(attrs);

  view! {
    <Primitive element=html::div
      as_child=as_child
      attrs=merged_attrs
      node_ref=collection_ref
      on:mousedown=move |ev: MouseEvent| {
        if let Some(on_mouse_down) = on_mouse_down {
          on_mouse_down.call(ev);
        }

        is_click_focus.set_value(true);
      }
      on:focus=move |ev: FocusEvent| {
        if let Some(on_focus) = on_focus {
          on_focus.call(ev.clone());
        }

        let is_keyboard_focus = !is_click_focus.get_value();

        if ev.target() == ev.current_target() && is_keyboard_focus && !is_tabbing_back_out.get() {
          let mut init = web_sys::CustomEventInit::new();
          init.bubbles(false).cancelable(true);

          let Ok(entry_focus_event) = web_sys::CustomEvent::new_with_event_init_dict("roving_focus_group.on_entry_focus", &init) else {
            return;
          };

          if entry_focus_event.default_prevented() {
            return;
          }

          let items = get_items.get();

          let items = items
            .iter()
            .filter_map(|(node, item)| {
              item.focusable.get().then_some((node.get()?, item))
            });

          let active_item = items.clone().find(|&(_, item)| item.active.get());
          let current_item = items.clone().find(|(_, item)| current_tab_stop_id.get().map(|id| id == item.id).unwrap_or(false));

          let candidate_nodes = items.map(Some)
            .chain([active_item, current_item].into_iter())
            .filter_map(|item| item.map(|(el, _)| el))
            .collect::<Vec<_>>();

          focus_first(&candidate_nodes, prevent_scroll_on_entry_focus.unwrap_or(false));
        }

        is_click_focus.set_value(false);
      }
      on:blur=move |ev: FocusEvent| {
        if let Some(on_blur) = on_blur {
          on_blur.call(ev);
        }

        set_is_tabbing_back_out.set(false);
      }
    >
      {children()}
    </Primitive>
  }
}

#[component]
pub(crate) fn RovingFocusGroupItem(
  #[prop(optional)] as_child: Option<bool>,
  #[prop(optional)] tab_stop_id: Option<MaybeSignal<String>>,
  #[prop(optional)] focusable: Option<MaybeSignal<bool>>,
  #[prop(optional)] active: Option<MaybeSignal<bool>>,
  #[prop(optional)] on_mouse_down: Option<Callback<MouseEvent>>,
  #[prop(optional)] on_focus: Option<Callback<FocusEvent>>,
  #[prop(optional)] on_key_down: Option<Callback<KeyboardEvent>>,
  #[prop(attrs)] attrs: Attributes,
  children: Children,
) -> impl IntoView {
  let RovingContextValue {
    orientation,
    direction,
    should_loop,
    current_tab_stop_id,
    on_item_focus,
    on_item_shift_tab,
    on_focusable_item_add,
    on_focusable_item_remove,
  } = use_context::<RovingContextValue>()
    .expect("RovingFocusGroupItem must be used in a RovingFocusGroup component");

  let id = Signal::derive(move || {
    tab_stop_id
      .as_ref()
      .map(|id| id.get())
      .unwrap_or(create_id().get())
  });

  let item_ref = create_collection_item_ref::<html::AnyElement, ItemData>(ItemData {
    id: id.get_untracked(),
    focusable: Signal::derive(move || focusable.map(|focusable| focusable.get()).unwrap_or(false)),
    active: Signal::derive(move || active.map(|active| active.get()).unwrap_or(false)),
  });

  let is_current_tab_stop = Signal::derive(move || current_tab_stop_id.get() == Some(id.get()));
  let get_items = use_collection_context::<ItemData, html::AnyElement>();

  Effect::new(move |_| {
    if focusable.map(|focusable| focusable.get()).unwrap_or(false) {
      on_focusable_item_add.call(());
      on_cleanup(move || {
        on_focusable_item_remove.call(());
      });
    }
  });

  let mut merged_attrs = vec![
    // (
    //   "tabindex",
    //   Signal::derive(move || if is_current_tab_stop.get() { 0 } else { -1 }).into_attribute(),
    // ),
    (
      "data-orientation",
      (move || orientation.get().map(|orientation| orientation.to_string())).into_attribute(),
    ),
  ];

  merged_attrs.extend(attrs);

  view! {
    <Primitive element=html::span
      as_child=as_child
      attrs=merged_attrs
      node_ref=item_ref
      on:mousedown=move |ev: MouseEvent| {
        if let Some(on_mouse_down) = on_mouse_down {
          on_mouse_down.call(ev.clone());
        }

        if !focusable.map(|focusable| focusable.get()).unwrap_or(false) {
          ev.prevent_default();
        } else {
          on_item_focus.call(id.get());
        }
      }
      on:focus=move |ev: FocusEvent| {
        if let Some(on_focus) = on_focus {
          on_focus.call(ev);
        }

        on_item_focus.call(id.get());
      }
      on:keydown=move |ev: KeyboardEvent| {
        if let Some(on_key_down) = on_key_down {
          on_key_down.call(ev.clone());
        }

        if ev.key() == "Tab" && ev.shift_key() {
          on_item_shift_tab.call(());
          return;
        }

        if ev.target() != ev.current_target() {
          return;
        }

        let focus_intent = get_focus_intent(&ev, orientation.get(), direction.get());

        if let Some(focus_intent) = focus_intent {
          if ev.meta_key() || ev.ctrl_key() || ev.alt_key() || ev.shift_key() {
            return;
          }

          ev.prevent_default();

          let items = get_items.get();
          let candidate_nodes = items.iter().filter_map(|(node, data)| data.focusable.get().then_some(node));

          let candidate_nodes = if focus_intent == FocusIntent::Last || focus_intent == FocusIntent::Prev {
            Either::Left(candidate_nodes.rev())
          } else {
            Either::Right(candidate_nodes)
          };

          let current_index = (focus_intent == FocusIntent::Prev || focus_intent == FocusIntent::Next)
            .then(|| {
              candidate_nodes
                .clone()
                .position(|node| {
                  (|| {
                    let node = node.get()?;

                    Some(ev.current_target()?
                      .dyn_ref::<web_sys::Element>()?
                      .eq(&node))
                  })().unwrap_or(false)
                })
            })
            .flatten();

          // let mut candidate_nodes = candidate_nodes
          let candidate_nodes = candidate_nodes
            .filter_map(|node| Some(node.get()?.into_any()))
            .collect::<Vec<_>>();

          let candidate_nodes = if let Some(current_index) = current_index {
            if should_loop.get().unwrap_or(false) {
              let len = candidate_nodes.len();
              // (&mut candidate_nodes).rotate_right((current_index + 1) % len); // might need in the future? may cause weird behavior if not added back

              // &candidate_nodes
              &candidate_nodes[(current_index + 1) % len..]
            } else {
              &candidate_nodes[(current_index + 1)..]
            }
          } else {
            &candidate_nodes
          };

          focus_first(candidate_nodes, false);
        }
      }
    >
      {children()}
    </Primitive>
  }
}

fn get_direction_aware_key(key: &str, direction: Option<Direction>) -> &str {
  match direction {
    Some(direction) => match direction {
      Direction::LeftToRight => key,
      _ => match key {
        "ArrowLeft" => "ArrowRight",
        "ArrowRight" => "ArrowLeft",
        _ => key,
      },
    },
    None => key,
  }
}

#[derive(PartialEq, Debug)]
enum FocusIntent {
  First,
  Last,
  Prev,
  Next,
}

fn get_focus_intent(
  ev: &web_sys::KeyboardEvent,
  orientation: Option<Orientation>,
  direction: Option<Direction>,
) -> Option<FocusIntent> {
  let key = ev.key();
  let key = get_direction_aware_key(&key, direction);

  match orientation {
    Some(Orientation::Vertical) if key == "ArrowLeft" || key == "ArrowRight" => None,
    Some(Orientation::Horizontal) if key == "ArrowUp" || key == "ArrowDown" => None,
    _ => match key {
      "ArrowLeft" | "ArrowUp" => Some(FocusIntent::Prev),
      "ArrowRight" | "ArrowDown" => Some(FocusIntent::Next),
      "PageUp" | "Home" => Some(FocusIntent::First),
      "PageDown" | "End" => Some(FocusIntent::Last),
      _ => None,
    },
  }
}

fn focus_first(candidates: &[HtmlElement<AnyElement>], prevent_scroll: bool) {
  let previously_focused = document().active_element();

  for candidate in candidates {
    let candidate_el: &web_sys::Element = candidate;

    if Some(candidate_el) == previously_focused.as_ref() {
      return;
    }

    _ = candidate.focus();

    if document().active_element() != previously_focused {
      return;
    }
  }
}
