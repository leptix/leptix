use std::collections::HashMap;

use leptos::{html::AnyElement, *};
use web_sys::KeyboardEvent;

use wasm_bindgen::JsCast;

use crate::{
  components::{
    collapsible::{CollapsibleContent, CollapsibleRoot, CollapsibleTrigger},
    collection::use_collection_context,
    primitive::Primitive,
  },
  util::{
    create_controllable_signal::{create_controllable_signal, CreateControllableSignalProps},
    create_id::create_id,
    Direction, Orientation,
  },
  Attributes,
};

use super::collection::CollectionContextValue;

pub enum AccordionKind {
  Single {
    value: Option<MaybeSignal<String>>,
    default_value: Option<MaybeSignal<String>>,
    on_value_change: Option<Callback<String>>,
    collapsible: Option<MaybeSignal<bool>>,
  },
  Multiple {
    value: Option<MaybeSignal<Vec<String>>>,
    default_value: Option<MaybeSignal<Vec<String>>>,
    on_value_change: Option<Callback<Vec<String>>>,
  },
}

#[derive(Clone)]
struct AccordionContextValue {
  value: Signal<Vec<String>>,
  on_item_open: Callback<String>,
  on_item_close: Callback<String>,
}

#[derive(Clone)]
struct AccordionCollapsibleContextValue {
  collapsible: Signal<bool>,
}

#[component]
pub fn AccordionRoot(
  kind: AccordionKind,
  #[prop(attrs)] attrs: Attributes,
  #[prop(optional)] node_ref: NodeRef<AnyElement>,
  children: Children,
) -> impl IntoView {
  provide_context(
    CollectionContextValue::<AccordionCollectionItem, AnyElement> {
      collection_ref: NodeRef::new(),
      item_map: RwSignal::new(HashMap::new()),
    },
  );

  match kind {
    AccordionKind::Single {
      value,
      default_value,
      on_value_change,
      collapsible,
    } => view! {
      <AccordionSingle
        attrs=attrs
        node_ref=node_ref
        value=value
        default_value=default_value
        on_value_change=on_value_change
        collapsible=collapsible
      >
        {children()}
      </AccordionSingle>
    },
    AccordionKind::Multiple {
      value,
      default_value,
      on_value_change,
    } => view! {
      <AccordionMultiple
        attrs=attrs
        node_ref=node_ref
        value=value
        default_value=default_value
        on_value_change=on_value_change
      >
        {children()}
      </AccordionMultiple>
    },
  }
}

#[component]
fn AccordionSingle(
  #[prop(optional_no_strip)] value: Option<MaybeSignal<String>>,
  #[prop(optional_no_strip)] default_value: Option<MaybeSignal<String>>,
  #[prop(optional_no_strip)] on_value_change: Option<Callback<String>>,
  #[prop(optional_no_strip)] collapsible: Option<MaybeSignal<bool>>,

  #[prop(attrs)] attrs: Attributes,
  #[prop(optional)] node_ref: NodeRef<AnyElement>,
  children: Children,
) -> impl IntoView {
  let (value, set_value) = create_controllable_signal(CreateControllableSignalProps {
    value: Signal::derive(move || value.clone().map(|value| value.get())),
    default_value: Signal::derive(move || {
      default_value
        .clone()
        .map(|default_value| default_value.get())
    }),
    on_change: Callback::new(move |value| {
      if let Some(on_value_change) = on_value_change {
        on_value_change.call(value);
      }
    }),
  });

  let set_on_item_open = set_value.clone();
  let set_on_item_close = set_value.clone();

  provide_context(AccordionContextValue {
    value: Signal::derive(move || value.get().map(|value| vec![value]).unwrap_or_default()),
    on_item_open: Callback::new(move |value| {
      set_on_item_open.set(value);
    }),
    on_item_close: Callback::new(move |_| {
      if collapsible
        .map(|collapsible| collapsible.get())
        .unwrap_or(false)
      {
        set_on_item_close.set(String::new());
      }
    }),
  });

  provide_context(AccordionCollapsibleContextValue {
    collapsible: Signal::derive(move || {
      collapsible
        .map(|collapsible| collapsible.get())
        .unwrap_or(false)
    }),
  });

  view! {
    <Accordion
      node_ref=node_ref
      attrs=attrs
    >
      {children()}
    </Accordion>
  }
}

#[component]
fn AccordionMultiple(
  #[prop(optional_no_strip)] value: Option<MaybeSignal<Vec<String>>>,
  #[prop(optional_no_strip)] default_value: Option<MaybeSignal<Vec<String>>>,
  #[prop(optional_no_strip)] on_value_change: Option<Callback<Vec<String>>>,

  #[prop(attrs)] attrs: Attributes,
  #[prop(optional)] node_ref: NodeRef<AnyElement>,
  children: Children,
) -> impl IntoView {
  let controllable_value = value.clone();
  let controllable_default_value = default_value.clone();
  let (value, set_value) = create_controllable_signal(CreateControllableSignalProps {
    value: Signal::derive(move || controllable_value.as_ref().map(|value| value.get())),
    default_value: Signal::derive(move || {
      controllable_default_value
        .as_ref()
        .map(|default_value| default_value.get())
    }),
    on_change: Callback::new(move |value| {
      if let Some(on_value_change) = on_value_change {
        on_value_change.call(value);
      }
    }),
  });

  let set_on_item_open = set_value.clone();
  let set_on_item_close = set_value.clone();

  provide_context(AccordionContextValue {
    value: Signal::derive(move || value.get().unwrap_or_default()),
    on_item_open: Callback::new(move |value| {
      set_on_item_open.update(|prev| {
        if let Some(prev) = prev {
          prev.push(value);
        } else {
          *prev = Some(vec![]);
        }
      });
    }),
    on_item_close: Callback::new(move |value| {
      set_on_item_close.update(|prev| {
        if let Some(prev) = prev {
          if let Some(position) = prev.iter().position(|prev_value| prev_value.eq(&value)) {
            prev.remove(position);
          }
        } else {
          *prev = Some(vec![]);
        }
      });
    }),
  });

  provide_context(AccordionCollapsibleContextValue {
    collapsible: Signal::derive(|| true),
  });

  view! {
    <Accordion
      node_ref=node_ref
      attrs=attrs
    >
      {children()}
    </Accordion>
  }
}

#[derive(Clone)]
struct AccordionStateContextValue {
  disabled: Signal<bool>,
  orientation: Signal<Orientation>,
  direction: Signal<Option<Direction>>,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord)]
struct AccordionCollectionItem;

#[component]
fn Accordion(
  #[prop(optional)] disabled: Option<MaybeSignal<bool>>,
  #[prop(optional)] orientation: Option<MaybeSignal<Orientation>>,
  #[prop(optional)] direction: Option<MaybeSignal<Direction>>,
  #[prop(optional)] on_key_down: Option<Callback<KeyboardEvent>>,

  #[prop(attrs)] attrs: Attributes,
  #[prop(optional)] node_ref: NodeRef<AnyElement>,
  children: Children,
) -> impl IntoView {
  let get_items = use_collection_context::<AccordionCollectionItem, AnyElement>();

  let is_direction_left_to_right = Signal::derive(move || {
    direction
      .map(|direction| direction.get() == Direction::LeftToRight)
      .unwrap_or(false)
  });

  provide_context(AccordionStateContextValue {
    disabled: Signal::derive(move || disabled.map(|disabled| disabled.get()).unwrap_or(false)),
    orientation: Signal::derive(move || {
      orientation
        .map(|orientation| orientation.get())
        .unwrap_or(Orientation::Vertical)
    }),
    direction: Signal::derive(move || direction.map(|direction| direction.get())),
  });

  let mut merged_attrs = vec![(
    "data-orientation",
    Signal::derive(move || {
      orientation
        .map(|orientation| orientation.get())
        .unwrap_or(Orientation::Vertical)
        .to_string()
    })
    .into_attribute(),
  )];

  merged_attrs.extend(attrs.into_iter());

  view! {
    <Primitive
      element=html::div
      attrs=merged_attrs
      node_ref=node_ref
      on:keydown=move |ev: KeyboardEvent| {
        if let Some(on_key_down) = on_key_down {
          on_key_down.call(ev.clone());
        }

        if disabled.map(|disabled| disabled.get()).unwrap_or(false) == false {
          return;
        }

        (|| {
          let target = ev.target()?;
          let target_el = target.dyn_ref::<web_sys::HtmlButtonElement>()?;
          let items = get_items.get();

          let triggers = items.iter().filter_map(|(node, _)| {
            let node = node.get()?;
            let node = node.dyn_ref::<web_sys::HtmlButtonElement>()?;

            Some((!node.disabled()).then_some(node.clone())).flatten()
          }).collect::<Vec<_>>();

          let trigger_index = triggers.iter().position(|item| item.eq(target_el))?;
          let trigger_count = triggers.len();

          ev.prevent_default();

          let mut next_index = trigger_index;

          let home_index = 0;
          let end_index = 0;

          match ev.key().as_str() {
            "Home" => {
              next_index = home_index;
            }
            "End" => {
              next_index = end_index;
            }
            "ArrowRight" => {
              if orientation.map(|orientation| orientation.get() == Orientation::Horizontal).unwrap_or(false) {
                if is_direction_left_to_right.get() {
                  next_index = trigger_index + 1;

                  if next_index > end_index {
                    next_index = home_index;
                  }
                } else {
                  next_index = trigger_index - 1;

                  if next_index < home_index {
                    next_index = end_index;
                  }
                }
              }
            }
            "ArrowDown" => {
              if orientation.map(|orientation| orientation.get() == Orientation::Vertical).unwrap_or(false) {
                next_index = trigger_index + 1;

                if next_index > end_index {
                  next_index = home_index;
                }
              }
            }
            "ArrowLeft" => {
              if orientation.map(|orientation| orientation.get() == Orientation::Horizontal).unwrap_or(false) {
                if is_direction_left_to_right.get() {
                  next_index = trigger_index - 1;

                  if next_index < home_index {
                    next_index = end_index;
                  }
                } else {
                  next_index = trigger_index + 1;

                  if next_index > end_index {
                    next_index = home_index;
                  }
                }
              }
            }
            "ArrowUp" => {
              if orientation.map(|orientation| orientation.get() == Orientation::Vertical).unwrap_or(false) {
                next_index = trigger_index - 1;

                if next_index < home_index {
                  next_index = end_index;
                }
              }
            }
            _ => {}
          }

          let clamped_index = next_index % trigger_count;
          if let Some(trigger) = triggers.get(clamped_index) {
            _ = trigger.focus();
          }

          Some(())
        })();
      }
    >
      {children()}
    </Primitive>
  }
}

#[derive(Clone)]
struct AccordionItemContextValue {
  open: Signal<bool>,
  disabled: Signal<bool>,
  trigger_id: Signal<String>,
}

#[component]
pub fn AccordionItem(
  #[prop(optional)] disabled: Option<MaybeSignal<bool>>,
  value: MaybeSignal<String>,
  #[prop(attrs)] attrs: Attributes,
  #[prop(optional)] node_ref: NodeRef<AnyElement>,
  children: Children,
) -> impl IntoView {
  let state_context = use_context::<AccordionStateContextValue>()
    .expect("AccordionItem must be in an Accordion component");
  let value_context = use_context::<AccordionContextValue>()
    .expect("AccordionItem must be in an AccordionRoot component");

  let trigger_id = create_id();
  let is_open_value = value.clone();
  let is_open = Signal::derive(move || {
    value_context
      .value
      .get()
      .iter()
      .find(|item| (*item).eq(&is_open_value.get()) && !is_open_value.get().is_empty())
      .is_some()
  });
  let is_disabled = Signal::derive(move || {
    state_context.disabled.get() || disabled.map(|disabled| disabled.get()).unwrap_or(false)
  });

  provide_context(AccordionItemContextValue {
    open: Signal::derive(move || is_open.get()),
    disabled: Signal::derive(move || is_disabled.get()),
    trigger_id: Signal::derive(move || trigger_id.get()),
  });

  let mut merged_attrs = vec![
    (
      "data-orientation",
      Signal::derive(move || state_context.orientation.get().to_string()).into_attribute(),
    ),
    (
      "data-state",
      Signal::derive(move || if is_open.get() { "open" } else { "closed" }).into_attribute(),
    ),
  ];

  merged_attrs.extend(attrs.into_iter());

  let open_value = value.clone();
  view! {
    <CollapsibleRoot
      attrs=merged_attrs
      open=Signal::derive(move || is_open.get()).into()
      disabled=Signal::derive(move || is_disabled.get()).into()
      node_ref=node_ref
      on_open_change=Callback::new(move |open| {
        if open {
          value_context.on_item_open.call(open_value.get());
        } else {
          value_context.on_item_close.call(open_value.get());
        }
      })
    >
      {children()}
    </CollapsibleRoot>
  }
}

#[component]
pub fn AccordionHeader(
  #[prop(attrs)] attrs: Attributes,
  #[prop(optional)] node_ref: NodeRef<AnyElement>,
  children: Children,
) -> impl IntoView {
  let state_context = use_context::<AccordionStateContextValue>()
    .expect("AccordionHeader must be in an Accordion component");
  let item_context = use_context::<AccordionItemContextValue>()
    .expect("AccordionHeader must be in an AccordionRoot component");

  let mut merged_attrs = vec![
    (
      "data-orientation",
      Signal::derive(move || state_context.orientation.get().to_string()).into_attribute(),
    ),
    (
      "data-state",
      Signal::derive(move || {
        if item_context.open.get() {
          "open"
        } else {
          "closed"
        }
      })
      .into_attribute(),
    ),
    (
      "data-disabled",
      Signal::derive(move || item_context.disabled.get()).into_attribute(),
    ),
  ];

  merged_attrs.extend(attrs.into_iter());

  view! {
    <Primitive
      element=html::h3
      attrs=merged_attrs
      node_ref=node_ref
    >
      {children()}
    </Primitive>
  }
}

#[component]
pub fn AccordionTrigger(
  #[prop(attrs)] attrs: Attributes,
  #[prop(optional)] node_ref: NodeRef<AnyElement>,
  children: Children,
) -> impl IntoView {
  let state_context = use_context::<AccordionStateContextValue>()
    .expect("AccordionTrigger must be in an Accordion component");
  let item_context = use_context::<AccordionItemContextValue>()
    .expect("AccordionTrigger must be in an AccordionRoot component");
  let collapsible_context = use_context::<AccordionCollapsibleContextValue>()
    .expect("AccordionTrigger must be in an AccordionRoot component");

  let mut merged_attrs = vec![
    (
      "data-orientation",
      Signal::derive(move || state_context.orientation.get().to_string()).into_attribute(),
    ),
    (
      "id",
      Signal::derive(move || item_context.trigger_id.get()).into_attribute(),
    ),
    (
      "aria-disabled",
      Signal::derive(move || item_context.open.get() && !collapsible_context.collapsible.get())
        .into_attribute(),
    ),
  ];

  merged_attrs.extend(attrs.into_iter());

  view! {
    <CollapsibleTrigger
      node_ref=node_ref
      attrs=merged_attrs
    >
      {children()}
    </CollapsibleTrigger>
  }
}

#[component]
pub fn AccordionContent(
  #[prop(attrs)] attrs: Attributes,
  #[prop(optional)] node_ref: NodeRef<AnyElement>,
  children: ChildrenFn,
) -> impl IntoView {
  let state_context = use_context::<AccordionStateContextValue>()
    .expect("AccordionTrigger must be in an Accordion component");
  let item_context = use_context::<AccordionItemContextValue>()
    .expect("AccordionTrigger must be in an AccordionRoot component");

  Effect::new(move |_| {
    let Some(node) = node_ref.get() else {
      return;
    };

    _ = node
      .style(
        "--primitive-accordion-content-width",
        "var(--primitive-collapsible-content-width)",
      )
      .style(
        "--primitive-accordion-content-height",
        "var(--primitive-collapsible-content-height)",
      );
  });

  let mut merged_attrs = vec![
    (
      "data-orientation",
      Signal::derive(move || state_context.orientation.get().to_string()).into_attribute(),
    ),
    (
      "aria-labelledby",
      Signal::derive(move || item_context.trigger_id.get()).into_attribute(),
    ),
    ("role", "region".into_attribute()),
  ];

  merged_attrs.extend(attrs);

  view! {
    <CollapsibleContent
      node_ref=node_ref
      attrs=merged_attrs
    >
      {children()}
    </CollapsibleContent>
  }
}
