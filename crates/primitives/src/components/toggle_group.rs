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
    value: Option<MaybeSignal<String>>,
    default_value: Option<MaybeSignal<String>>,
    on_value_change: Option<Callback<String>>,
  },
  Multiple {
    value: Option<MaybeSignal<Vec<String>>>,
    default_value: Option<MaybeSignal<Vec<String>>>,
    on_value_change: Option<Callback<Vec<String>>>,
  },
}

#[component]
pub fn ToggleGroupRoot(
  kind: ToggleGroupKind,

  #[prop(optional)] disabled: Option<MaybeSignal<bool>>,
  #[prop(optional)] roving_focus: Option<MaybeSignal<bool>>,
  #[prop(optional)] should_loop: Option<MaybeSignal<bool>>,
  #[prop(optional)] orientation: Option<MaybeSignal<Orientation>>,
  #[prop(optional)] direction: Option<MaybeSignal<Direction>>,

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
  #[prop(optional_no_strip)] disabled: Option<MaybeSignal<bool>>,
  #[prop(optional_no_strip)] roving_focus: Option<MaybeSignal<bool>>,
  #[prop(optional_no_strip)] should_loop: Option<MaybeSignal<bool>>,
  #[prop(optional_no_strip)] orientation: Option<MaybeSignal<Orientation>>,
  #[prop(optional_no_strip)] direction: Option<MaybeSignal<Direction>>,

  #[prop(optional_no_strip)] value: Option<MaybeSignal<String>>,
  #[prop(optional_no_strip)] default_value: Option<MaybeSignal<String>>,
  #[prop(optional_no_strip)] on_value_change: Option<Callback<String>>,

  #[prop(attrs)] attrs: Attributes,
  #[prop(optional)] node_ref: NodeRef<AnyElement>,
  children: ChildrenFn,
) -> impl IntoView {
  let (value, set_value) = create_controllable_signal(CreateControllableSignalProps {
    value: Signal::derive(move || value.as_ref().map(|value| value.get())),
    default_value: Signal::derive(move || {
      default_value
        .as_ref()
        .map(|default_value| default_value.get())
    }),
    on_change: Callback::new(move |value| {
      if let Some(on_value_change) = on_value_change {
        on_value_change.call(value);
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
  #[prop(optional_no_strip)] disabled: Option<MaybeSignal<bool>>,
  #[prop(optional_no_strip)] roving_focus: Option<MaybeSignal<bool>>,
  #[prop(optional_no_strip)] should_loop: Option<MaybeSignal<bool>>,
  #[prop(optional_no_strip)] orientation: Option<MaybeSignal<Orientation>>,
  #[prop(optional_no_strip)] direction: Option<MaybeSignal<Direction>>,

  #[prop(optional_no_strip)] value: Option<MaybeSignal<Vec<String>>>,
  #[prop(optional_no_strip)] default_value: Option<MaybeSignal<Vec<String>>>,
  #[prop(optional_no_strip)] on_value_change: Option<Callback<Vec<String>>>,

  #[prop(attrs)] attrs: Attributes,
  #[prop(optional)] node_ref: NodeRef<AnyElement>,
  children: ChildrenFn,
) -> impl IntoView {
  let (value, set_value) = create_controllable_signal(CreateControllableSignalProps {
    value: Signal::derive(move || value.as_ref().map(|value| value.get())),
    default_value: Signal::derive(move || {
      default_value
        .as_ref()
        .map(|default_value| default_value.get())
    }),
    on_change: Callback::new(move |value| {
      if let Some(on_value_change) = on_value_change {
        on_value_change.call(value);
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
          *value = Some(vec![item]);
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
  #[prop(optional_no_strip)] disabled: Option<MaybeSignal<bool>>,
  #[prop(optional_no_strip)] roving_focus: Option<MaybeSignal<bool>>,
  #[prop(optional_no_strip)] should_loop: Option<MaybeSignal<bool>>,
  #[prop(optional_no_strip)] orientation: Option<MaybeSignal<Orientation>>,
  #[prop(optional_no_strip)] direction: Option<MaybeSignal<Direction>>,

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
        ("dir", (move ||
          direction.map(|direction| direction.get()).unwrap_or_default().to_string())
        .into_attribute())
      ]);

      if roving_focus.map(|roving_focus| roving_focus.get()).unwrap_or(true) {
        view! {
          <RovingFocusGroup
            as_child=true
            orientation=Signal::derive(move || orientation.map(|orientation| orientation.get()).unwrap_or_default()).into()
            direction=Signal::derive(move || direction.map(|direction| direction.get()).unwrap_or_default()).into()
            should_loop=Signal::derive(move || should_loop.map(|should_loop| should_loop.get()).unwrap_or(true)).into()
          >
            <Primitive
              element=html::div
              attrs=merged_attrs
              node_ref=node_ref
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
            node_ref=node_ref
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
  #[prop(optional)] disabled: Option<MaybeSignal<bool>>,
  value: MaybeSignal<String>,

  #[prop(attrs)] attrs: Attributes,
  #[prop(optional)] node_ref: NodeRef<AnyElement>,
  children: ChildrenFn,
) -> impl IntoView {
  let value_context = use_context::<ToggleGroupValueContextValue>()
    .expect("ToggleGroupItem must be in a ToggleGroupRoot component");
  let state_context = use_context::<ToggleGroupStateContextValue>()
    .expect("ToggleGroupItem must be in a ToggleGroupRoot component");

  let is_pressed_value = value.clone();
  let is_pressed =
    Signal::derive(move || value_context.value.get().contains(&is_pressed_value.get()));

  let is_disabled = Signal::derive(move || {
    state_context.disabled.get() || disabled.map(|disabled| disabled.get()).unwrap_or(false)
  });

  let inner_value = value.clone();
  view! {
    {move || {
      let children = children.clone();
      let mut merged_attrs = attrs.clone();

      if value_context.kind == ToggleGroupValueKind::Single {
        merged_attrs.extend([("role", "radio".into_attribute()), ("aria-checked", Signal::derive(move || is_pressed.get().to_string()).into_attribute())].into_iter());
      }

      let on_pressed_value = inner_value.clone();

      if state_context.roving_focus.get() {
        view! {
          <RovingFocusGroupItem
            as_child=true
            focusable=Signal::derive(move || !is_disabled.get()).into()
            active=Signal::derive(move || is_pressed.get()).into()
          >
            <ToggleRoot
              disabled=Signal::derive(move || is_disabled.get()).into()
              pressed=Signal::derive(move || is_pressed.get()).into()
              attrs=merged_attrs
              node_ref=node_ref
              on_pressed_changed=Callback::new(move |pressed| {
                if pressed {
                  value_context.on_item_activate.call(on_pressed_value.get());
                } else {
                  value_context.on_item_deactivate.call(on_pressed_value.get());
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
            disabled=Signal::derive(move || is_disabled.get()).into()
            pressed=Signal::derive(move || is_pressed.get()).into()
            attrs=merged_attrs
            node_ref=node_ref
            on_pressed_changed=Callback::new(move |pressed| {
              if pressed {
                value_context.on_item_activate.call(on_pressed_value.get());
              } else {
                value_context.on_item_deactivate.call(on_pressed_value.get());
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
