use leptos::{html::AnyElement, *};

use crate::{
  components::{
    primitive::Primitive,
    roving_focus::{RovingFocusGroup, RovingFocusGroupItem},
    toggle::ToggleRoot,
  },
  util::{
    create_controllable_signal::{create_controllable_signal, CreateControllableSignalProps},
    Direction, Orientation,
  },
  Attributes,
};

pub enum ToggleGroupKind {
  Single {
    value: Option<Signal<String>>,
    default_value: Option<Signal<String>>,
    on_value_change: Option<Callback<String>>,
  },
  Multiple {
    value: Option<Signal<Vec<String>>>,
    default_value: Option<Signal<Vec<String>>>,
    on_value_change: Option<Callback<Vec<String>>>,
  },
}

#[component]
pub fn ToggleGroupRoot(
  kind: ToggleGroupKind,

  #[prop(optional)] disabled: Option<Signal<bool>>,
  #[prop(optional)] roving_focus: Option<Signal<bool>>,
  #[prop(optional)] should_loop: Option<Signal<bool>>,
  #[prop(optional)] orientation: Option<Signal<Orientation>>,
  #[prop(optional)] direction: Option<Signal<Direction>>,

  #[prop(attrs)] attrs: Attributes,
  #[prop(optional)] node_ref: NodeRef<AnyElement>,
  children: ChildrenFn,
) -> impl IntoView {
  match kind {
    ToggleGroupKind::Single {
      value,
      default_value,
      on_value_change,
    } => view! {
      <ToggleGroupSingle
        disabled=disabled
        roving_focus=roving_focus
        should_loop=should_loop
        orientation=orientation
        direction=direction
        value=value
        default_value=default_value
        on_value_change=on_value_change
        attrs=attrs
        node_ref=node_ref
      >
        {children()}
      </ToggleGroupSingle>
    },
    ToggleGroupKind::Multiple {
      value,
      default_value,
      on_value_change,
    } => view! {
      <ToggleGroupMultiple
        disabled=disabled
        roving_focus=roving_focus
        should_loop=should_loop
        orientation=orientation
        direction=direction
        value=value
        default_value=default_value
        on_value_change=on_value_change
        attrs=attrs
        node_ref=node_ref
      >
        {children()}
      </ToggleGroupMultiple>
    },
  }
}

#[derive(Clone, PartialEq)]
enum ToggleGroupValueKind {
  Single,
  Multiple,
}

#[derive(Clone)]
struct ToggleGroupValueContextValue {
  kind: ToggleGroupValueKind,
  value: Signal<Vec<String>>,
  on_item_activate: Callback<String>,
  on_item_deactivate: Callback<String>,
}

#[component]
fn ToggleGroupSingle(
  #[prop(optional_no_strip)] disabled: Option<Signal<bool>>,
  #[prop(optional_no_strip)] roving_focus: Option<Signal<bool>>,
  #[prop(optional_no_strip)] should_loop: Option<Signal<bool>>,
  #[prop(optional_no_strip)] orientation: Option<Signal<Orientation>>,
  #[prop(optional_no_strip)] direction: Option<Signal<Direction>>,

  #[prop(optional_no_strip)] value: Option<Signal<String>>,
  #[prop(optional_no_strip)] default_value: Option<Signal<String>>,
  #[prop(optional_no_strip)] on_value_change: Option<Callback<String>>,

  #[prop(attrs)] attrs: Attributes,
  #[prop(optional)] node_ref: NodeRef<AnyElement>,
  children: ChildrenFn,
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

  let set_on_item_activate = set_value.clone();
  let set_on_item_deactivate = set_value.clone();
  provide_context(ToggleGroupValueContextValue {
    kind: ToggleGroupValueKind::Single,
    value: Signal::derive(move || value.get().map(|value| vec![value]).unwrap_or_default()),
    on_item_activate: Callback::new(move |item| {
      set_on_item_activate.set(item);
    }),
    on_item_deactivate: Callback::new(move |_| set_on_item_deactivate.set(String::new())),
  });

  view! {
    <ToggleGroup
      disabled=disabled
      roving_focus=roving_focus
      should_loop=should_loop
      orientation=orientation
      direction=direction
      node_ref=node_ref
      attrs=attrs
    >
      {children()}
    </ToggleGroup>
  }
}

#[component]
fn ToggleGroupMultiple(
  #[prop(optional_no_strip)] disabled: Option<Signal<bool>>,
  #[prop(optional_no_strip)] roving_focus: Option<Signal<bool>>,
  #[prop(optional_no_strip)] should_loop: Option<Signal<bool>>,
  #[prop(optional_no_strip)] orientation: Option<Signal<Orientation>>,
  #[prop(optional_no_strip)] direction: Option<Signal<Direction>>,

  #[prop(optional_no_strip)] value: Option<Signal<Vec<String>>>,
  #[prop(optional_no_strip)] default_value: Option<Signal<Vec<String>>>,
  #[prop(optional_no_strip)] on_value_change: Option<Callback<Vec<String>>>,

  #[prop(attrs)] attrs: Attributes,
  #[prop(optional)] node_ref: NodeRef<AnyElement>,
  children: ChildrenFn,
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

  let set_on_item_activate = set_value.clone();
  let set_on_item_deactivate = set_value.clone();
  provide_context(ToggleGroupValueContextValue {
    kind: ToggleGroupValueKind::Multiple,
    value: Signal::derive(move || value.get().unwrap_or_default()),
    on_item_activate: Callback::new(move |item| {
      set_on_item_activate.update(|value| {
        if let Some(value) = value {
          value.push(item);
        } else {
          *value = Some(vec![]);
        }
      });
    }),
    on_item_deactivate: Callback::new(move |item| {
      set_on_item_deactivate.update(|value| {
        if let Some(value) = value {
          *value = value
            .iter()
            .filter_map(|value| (*value != item).then_some(value.to_string()))
            .collect::<Vec<_>>();
        } else {
          *value = Some(vec![]);
        }
      })
    }),
  });

  view! {
    <ToggleGroup
      disabled=disabled
      roving_focus=roving_focus
      should_loop=should_loop
      orientation=orientation
      direction=direction
      node_ref=node_ref
      attrs=attrs
    >
      {children()}
    </ToggleGroup>
  }
}

#[derive(Clone)]
struct ToggleGroupStateContextValue {
  disabled: Signal<bool>,
  roving_focus: Signal<bool>,
}

#[component]
fn ToggleGroup(
  #[prop(optional_no_strip)] disabled: Option<Signal<bool>>,
  #[prop(optional_no_strip)] roving_focus: Option<Signal<bool>>,
  #[prop(optional_no_strip)] should_loop: Option<Signal<bool>>,
  #[prop(optional_no_strip)] orientation: Option<Signal<Orientation>>,
  #[prop(optional_no_strip)] direction: Option<Signal<Direction>>,

  #[prop(attrs)] attrs: Attributes,
  #[prop(optional)] node_ref: NodeRef<AnyElement>,
  children: ChildrenFn,
) -> impl IntoView {
  provide_context(ToggleGroupStateContextValue {
    roving_focus: Signal::derive(move || {
      roving_focus
        .map(|roving_focus| roving_focus.get())
        .unwrap_or(true)
    }),
    disabled: Signal::derive(move || disabled.map(|disabled| disabled.get()).unwrap_or(false)),
  });

  view! {
    {move || {
      let children = children.clone();
      let mut merged_attrs = attrs.clone();

      merged_attrs.extend([
        ("role", "group".into_attribute()),
        ("dir", Signal::derive(move || {
          match direction.map(|direction| direction.get()).unwrap_or(Direction::LeftToRight) {
            Direction::LeftToRight => "ltr",
            Direction::RightToLeft => "rtl",
          }
        })
        .into_attribute())
      ]);

      if roving_focus.map(|roving_focus| roving_focus.get()).unwrap_or(true) {
        view! {
          <RovingFocusGroup
            // as_child=true
            orientation=Signal::derive(move || orientation.map(|orientation| orientation.get()).unwrap_or(Orientation::Horizontal))
            direction=Signal::derive(move || direction.map(|direction| direction.get()).unwrap_or(Direction::LeftToRight))
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
      } else {
        view! {
          <Primitive
            element=html::div
            attrs=merged_attrs
            node_ref=Some(node_ref)
          >
            {children()}
          </Primitive>
        }
      }
    }}
  }
}

#[component]
pub fn ToggleGroupItem(
  #[prop(optional)] disabled: Option<Signal<bool>>,
  value: Signal<String>,

  #[prop(attrs)] attrs: Attributes,
  #[prop(optional)] node_ref: NodeRef<AnyElement>,
  children: ChildrenFn,
) -> impl IntoView {
  let value_context = use_context::<ToggleGroupValueContextValue>()
    .expect("ToggleGroupItem must be in a ToggleGroupRoot component");
  let state_context = use_context::<ToggleGroupStateContextValue>()
    .expect("ToggleGroupItem must be in a ToggleGroupRoot component");

  let is_pressed = Signal::derive(move || value_context.value.get().contains(&value.get()));

  let is_disabled = Signal::derive(move || {
    state_context.disabled.get() || disabled.map(|disabled| disabled.get()).unwrap_or(false)
  });

  view! {
    {move || {
      let children = children.clone();
      let mut merged_attrs = attrs.clone();

      if value_context.kind == ToggleGroupValueKind::Single {
        merged_attrs.extend([("role", "radio".into_attribute()), ("aria-checked", Signal::derive(move || is_pressed.get().to_string()).into_attribute())].into_iter());
      }

      if state_context.roving_focus.get() {
        view! {
          <RovingFocusGroupItem
            // as_child=true
            focusable=Signal::derive(move || is_disabled.get())
            active=Signal::derive(move || is_pressed.get())
          >
            <ToggleRoot
              attrs=merged_attrs
              node_ref=node_ref
              on_pressed_changed=Callback::new(move |pressed| {
                if pressed {
                  (value_context.on_item_activate)(value.get());
                } else {
                  (value_context.on_item_deactivate)(value.get());
                }
              })
            >
              {children()}
            </ToggleRoot>
          </RovingFocusGroupItem>
        }
      } else {
        view! {
          <ToggleRoot
            attrs=merged_attrs
            node_ref=node_ref
            on_pressed_changed=Callback::new(move |pressed| {
              if pressed {
                (value_context.on_item_activate)(value.get());
              } else {
                (value_context.on_item_deactivate)(value.get());
              }
            })
          >
            {children()}
          </ToggleRoot>
        }
      }
    }}
  }
}
