use std::{borrow::Cow, collections::HashMap};

use leptos::{
  either::Either,
  ev::EventDescriptor,
  html::{self, Div, Span},
  prelude::*,
};
use leptos_use::use_event_listener;
use wasm_bindgen::JsCast;
use web_sys::{Event, FocusEvent, KeyboardEvent, MouseEvent};

use crate::{
  collection::{use_collection_context, CollectionContextValue},
  primitive::Primitive,
  util::{
    create_controllable_signal::{create_controllable_signal, CreateControllableSignalProps},
    create_id::create_id,
  },
  Direction, Orientation,
};

use super::collection::use_collection_item_ref;

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
  should_loop: Signal<bool>,
  current_tab_stop_id: Signal<Option<String>>,
  on_item_focus: Callback<String>,
  on_item_shift_tab: Callback<()>,
  on_focusable_item_add: Callback<()>,
  on_focusable_item_remove: Callback<()>,
  focusable_items: RwSignal<i32>,
}

#[derive(Clone)]
struct OnEntryFocus;

impl EventDescriptor for OnEntryFocus {
  const BUBBLES: bool = false;
  type EventType = web_sys::Event;

  fn name(&self) -> Cow<'static, str> {
    "roving_focus_group.on_entry_focus".into()
  }

  fn event_delegation_key(&self) -> Cow<'static, str> {
    "$$$roving_focus_group.on_entry_focus".into()
  }
}

#[component]
pub(crate) fn RovingFocusGroup(
  #[prop(optional, into)] orientation: MaybeProp<Orientation>,
  #[prop(optional, into)] direction: MaybeProp<Direction>,
  #[prop(optional, into)] should_loop: MaybeSignal<bool>,
  #[prop(optional, into)] current_tab_stop_id: MaybeProp<String>,
  #[prop(optional, into)] default_current_tab_stop_id: MaybeProp<String>,
  #[prop(optional, into)] prevent_scroll_on_entry_focus: MaybeSignal<bool>,

  #[prop(default=Callback::new(|_|{}), into)] on_current_tab_stop_id_change: Callback<
    Option<String>,
  >,
  #[prop(default=Callback::new(|_|{}), into)] on_entry_focus: Callback<Event>,
  #[prop(default=Callback::new(|_|{}), into)] on_mouse_down: Callback<MouseEvent>,
  #[prop(default=Callback::new(|_|{}), into)] on_focus: Callback<FocusEvent>,
  #[prop(default=Callback::new(|_|{}), into)] on_blur: Callback<FocusEvent>,

  children: ChildrenFn,

  #[prop(optional, into)] as_child: MaybeProp<bool>,
) -> impl IntoView {
  let collection_ref = NodeRef::<Div>::new();

  provide_context(CollectionContextValue::<ItemData, _> {
    collection_ref,
    item_map: RwSignal::new(HashMap::new()),
  });

  let (current_tab_stop_id, set_current_tab_stop_id) =
    create_controllable_signal(CreateControllableSignalProps {
      value: Signal::derive(move || current_tab_stop_id.get()),
      default_value: Signal::derive(move || default_current_tab_stop_id.get()),
      on_change: Callback::new(move |value| on_current_tab_stop_id_change.run(Some(value))),
    });

  let (is_tabbing_back_out, set_is_tabbing_back_out) = signal(false);

  let get_items = use_collection_context::<ItemData, Div>();
  let is_click_focus = StoredValue::new(false);

  let focusable_items_count = RwSignal::new(0);

  // _ = use_event_listener(collection_ref, OnEntryFocus, move |ev: web_sys::Event| {
  //     on_entry_focus.run(ev);
  // });

  provide_context(RovingContextValue {
    orientation: Signal::derive(move || orientation.get()),
    direction: Signal::derive(move || direction.get()),
    should_loop: Signal::derive(move || should_loop.get()),
    current_tab_stop_id: Signal::derive(move || current_tab_stop_id.get()),
    on_item_focus: Callback::new(move |item| {
      set_current_tab_stop_id.set(item);
    }),
    on_item_shift_tab: Callback::new(move |_| {
      set_is_tabbing_back_out.set(true);
    }),
    on_focusable_item_add: Callback::new(move |_| {
      focusable_items_count.update(|count| {
        *count += 1;
      });
    }),
    on_focusable_item_remove: Callback::new(move |_| {
      _ = focusable_items_count.try_update(|count| {
        *count -= 1;
      });
    }),
    focusable_items: focusable_items_count,
  });

  view! {
    <Primitive
      element={html::div}
      node_ref={collection_ref}
      as_child={as_child}
      attr:data-orientation=move || orientation.get().map(|orientation| orientation.to_string())
      {..}
      tabindex=move || {
        if is_tabbing_back_out.get() || focusable_items_count.get() == 0 {
          -1
        } else {
          0
        }
      }
      on:mousedown=move |ev: MouseEvent| {
          on_mouse_down.run(ev);
        is_click_focus.set_value(true);
      }
      on:focus=move |ev: FocusEvent| {
          on_focus.run(ev.clone());

        let is_keyboard_focus = !is_click_focus.get_value();

        if ev.target() == ev.current_target() && is_keyboard_focus && !is_tabbing_back_out.get() {
          let mut init = web_sys::CustomEventInit::new();
          init.set_bubbles(false);
          init.set_cancelable(true);

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
            .filter_map(|item| item.and_then(|(el, _)| el.dyn_into::<web_sys::HtmlElement>().ok()))
            .collect::<Vec<_>>();

          focus_first(candidate_nodes.as_slice(), prevent_scroll_on_entry_focus.get());
        }

        is_click_focus.set_value(false);
      }
      on:blur=move |ev: FocusEvent| {
          on_blur.run(ev);
        set_is_tabbing_back_out.set(false);
      }
    >
      {children()}
    </Primitive>
  }
}

#[component]
pub(crate) fn RovingFocusGroupItem(
  #[prop(optional, into)] tab_stop_id: MaybeProp<String>,
  #[prop(optional, into)] focusable: MaybeSignal<bool>,
  #[prop(optional, into)] active: MaybeSignal<bool>,

  #[prop(default=Callback::new(|_|{}), into)] on_mouse_down: Callback<MouseEvent>,
  #[prop(default=Callback::new(|_|{}), into)] on_focus: Callback<FocusEvent>,
  #[prop(default=Callback::new(|_|{}), into)] on_key_down: Callback<KeyboardEvent>,

  #[prop(optional)] node_ref: NodeRef<Span>,
  children: ChildrenFn,

  #[prop(optional, into)] as_child: MaybeProp<bool>,
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
    focusable_items,
  } = use_context::<RovingContextValue>()
    .expect("RovingFocusGroupItem must be used in a RovingFocusGroup component");

  let id = Signal::derive(move || tab_stop_id.get().unwrap_or(create_id().get()));

  use_collection_item_ref::<Span, ItemData>(
    node_ref,
    ItemData {
      id: id.get_untracked(),
      focusable: Signal::derive(move || focusable.get()),
      active: Signal::derive(move || active.get()),
    },
  );

  let is_current_tab_stop = Signal::derive(move || current_tab_stop_id.get() == Some(id.get()));
  let get_items = use_collection_context::<ItemData, Span>();

  Effect::new(move |_| {
    if focusable.get() {
      focusable_items.update(|items| *items += 1);

      on_cleanup(move || {
        _ = focusable_items.try_update(|items| *items -= 1);
      });
    }
  });

  view! {
    <Primitive
      element={html::span}
      node_ref={node_ref}
      as_child={as_child}
      attr:data-orientation=move || orientation.get().map(|orientation| orientation.to_string())
      {..}
      tabindex=move || if is_current_tab_stop.get() { 0 } else { -1 }
      on:mousedown=move |ev: MouseEvent| {
        on_mouse_down.run(ev.clone());

        if !focusable.get() {
          ev.prevent_default();
        } else {
          on_item_focus.run(id.get());
        }
      }
      on:focus=move |ev: FocusEvent| {
          on_focus.run(ev);

        on_item_focus.run(id.get());
      }
      on:keydown=move |ev: KeyboardEvent| {
          on_key_down.run(ev.clone());

        if ev.key() == "Tab" && ev.shift_key() {
          on_item_shift_tab.run(());
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
            .filter_map(|node| node.get().and_then(|node| node.dyn_into::<web_sys::HtmlElement>().ok()))
            .collect::<Vec<_>>();

          let candidate_nodes = if let Some(current_index) = current_index {
            if should_loop.get() {
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

fn focus_first(candidates: &[web_sys::HtmlElement], prevent_scroll: bool) {
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
