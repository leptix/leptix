use leptos::{html::AnyElement, *};
use web_sys::{FocusEvent, KeyboardEvent, MouseEvent};

use crate::{
  components::{
    presence::create_presence,
    primitive::Primitive,
    roving_focus::{RovingFocusGroup, RovingFocusGroupItem},
  },
  util::{
    create_controllable_signal::{create_controllable_signal, CreateControllableSignalProps},
    create_id::create_id,
    Direction, Orientation,
  },
  Attributes,
};

#[derive(Clone)]
struct TabsContextValue {
  base_id: Signal<String>,
  value: Signal<Option<String>>,
  on_value_change: Callback<String>,
  orientation: Signal<Orientation>,
  direction: Signal<Direction>,
  activation_mode: Signal<ActivationMode>,
}

#[derive(Clone, Default, PartialEq)]
pub enum ActivationMode {
  #[default]
  Automatic,
  Manual,
}

#[component]
pub fn TabsRoot(
  #[prop(optional)] value: Option<Signal<String>>,
  #[prop(optional)] default_value: Option<Signal<String>>,
  #[prop(optional)] on_value_change: Option<Callback<String>>,
  #[prop(optional)] orientation: Option<Signal<Orientation>>,
  #[prop(optional)] direction: Option<Signal<Direction>>,
  #[prop(optional)] activation_mode: Option<Signal<ActivationMode>>,

  #[prop(attrs)] attrs: Attributes,
  #[prop(optional)] node_ref: NodeRef<AnyElement>,
  children: Children,
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

  provide_context(TabsContextValue {
    base_id: create_id(),
    value: Signal::derive(move || value.get()),
    on_value_change: Callback::new(move |value| {
      // logging::log!("setting tab value");
      set_value.set(value);
      // logging::log!("tab value set");
    }),
    direction: Signal::derive(move || {
      direction
        .map(|direction| direction.get())
        .unwrap_or_default()
    }),
    orientation: Signal::derive(move || {
      orientation
        .map(|orientation| orientation.get())
        .unwrap_or_default()
    }),
    activation_mode: Signal::derive(move || {
      activation_mode
        .map(|activation_mode| activation_mode.get())
        .unwrap_or_default()
    }),
  });

  view! {
    <Primitive
      element=html::div
      attrs=attrs
      node_ref=Some(node_ref)
    >
      {children()}
    </Primitive>
  }
}

#[component]
pub fn TabsList(
  #[prop(optional)] should_loop: Option<Signal<bool>>,

  #[prop(attrs)] attrs: Attributes,
  #[prop(optional)] node_ref: NodeRef<AnyElement>,
  children: Children,
) -> impl IntoView {
  let context =
    use_context::<TabsContextValue>().expect("TabsList must be used in a TabsRoot component");

  let mut merged_attrs = attrs.clone();
  merged_attrs.extend(
    [
      ("role", "tablist".into_attribute()),
      (
        "aria-orientation",
        Signal::derive(move || match context.orientation.get() {
          Orientation::Horizontal => "horizontal",
          Orientation::Vertical => "vertical",
        })
        .into_attribute(),
      ),
    ]
    .into_iter(),
  );

  view! {
    <RovingFocusGroup
      as_child=true
      orientation=Signal::derive(move || context.orientation.get())
      direction=Signal::derive(move || context.direction.get())
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
}

#[component]
pub fn TabsTrigger(
  #[prop(optional)] value: Signal<String>,
  #[prop(optional)] disabled: Option<Signal<bool>>,
  #[prop(optional)] on_mouse_down: Option<Callback<MouseEvent>>,
  #[prop(optional)] on_key_down: Option<Callback<KeyboardEvent>>,
  #[prop(optional)] on_focus: Option<Callback<FocusEvent>>,

  #[prop(attrs)] attrs: Attributes,
  #[prop(optional)] node_ref: NodeRef<AnyElement>,
  children: Children,
) -> impl IntoView {
  let context =
    use_context::<TabsContextValue>().expect("TabsTrigger must be used in a TabsRoot component");

  let trigger_id =
    Signal::derive(move || format!("{}-trigger-{}", context.base_id.get(), value.get()));
  let content_id =
    Signal::derive(move || format!("{}-content-{}", context.base_id.get(), value.get()));
  let is_selected = Signal::derive(move || context.value.get() == Some(value.get()));

  let mut merged_attrs = attrs.clone();
  merged_attrs.extend(
    [
      ("type", "button".into_attribute()),
      ("role", "tab".into_attribute()),
      (
        "aria-selected",
        Signal::derive(move || is_selected.get()).into_attribute(),
      ),
      (
        "aria-controls",
        Signal::derive(move || content_id.get()).into_attribute(),
      ),
      (
        "data-state",
        Signal::derive(move || {
          if is_selected.get() {
            "active"
          } else {
            "inactive"
          }
        })
        .into_attribute(),
      ),
      (
        "data-disabled",
        Signal::derive(move || disabled.map(|disabled| disabled.get().then_some("")))
          .into_attribute(),
      ),
      (
        "disabled",
        Signal::derive(move || disabled.map(|disabled| disabled.get())).into_attribute(),
      ),
      (
        "id",
        Signal::derive(move || trigger_id.get()).into_attribute(),
      ),
    ]
    .into_iter(),
  );

  view! {
    <RovingFocusGroupItem
      as_child=true
      focusable=Signal::derive(move || !disabled.map(|disabled| disabled.get()).unwrap_or(false))
      active=Signal::derive(move || is_selected.get())
    >
      <Primitive
        element=html::button
        attrs=merged_attrs
        node_ref=Some(node_ref)
        on:mousedown=move|ev: MouseEvent| {
          if let Some(on_mouse_down) = on_mouse_down {
            on_mouse_down(ev.clone());
          }

          // logging::log!("selecting tab");

          if !disabled.map(|disabled| disabled.get()).unwrap_or(false) && ev.button() == 0 && ev.ctrl_key() == false {
            // logging::log!("firing on_value_change");
            (context.on_value_change)(value.get());
            // logging::log!("fired on_value_change");
          } else {
            ev.prevent_default();
          }

          // logging::log!("tab click event done");
        }
        on:keydown=move |ev: KeyboardEvent| {
          if let Some(on_key_down) = on_key_down {
            on_key_down(ev.clone());
          }

          if [" ", "Enter"].contains(&ev.key().as_str()) {
            (context.on_value_change)(value.get());
          }
        }
        on:focus=move |ev: FocusEvent| {
          if let Some(on_focus) = on_focus {
            on_focus(ev.clone());
          }

          let is_automatic_activation = context.activation_mode.get() != ActivationMode::Manual;

          if !is_selected.get() && !disabled.map(|disabled| disabled.get()).unwrap_or(false) && is_automatic_activation {
            (context.on_value_change)(value.get());
          }
        }
      >
        {children()}
      </Primitive>
    </RovingFocusGroupItem>
  }
}

#[component]
pub fn TabsContent(
  #[prop(optional)] value: Signal<String>,
  #[prop(optional)] force_mount: Option<Signal<bool>>,

  #[prop(attrs)] attrs: Attributes,
  #[prop(optional)] node_ref: NodeRef<AnyElement>,
  children: ChildrenFn,
) -> impl IntoView {
  let context =
    use_context::<TabsContextValue>().expect("TabsContent must be used in a TabsRoot component");

  let trigger_id =
    Signal::derive(move || format!("{}-trigger-{}", context.base_id.get(), value.get()));
  let content_id =
    Signal::derive(move || format!("{}-content-{}", context.base_id.get(), value.get()));

  let is_selected = Signal::derive(move || context.value.get() == Some(value.get()));
  let is_mount_animation_prevented = store_value(is_selected.get());

  let is_present = Signal::derive(move || {
    is_selected.get()
      || force_mount
        .map(|force_mount| force_mount.get())
        .unwrap_or(false)
  });

  // let presence = create_presence(is_present);

  create_effect(move |_| {
    let Ok(animation_frame_handle) = request_animation_frame_with_handle(move || {
      is_mount_animation_prevented.set_value(false);
    }) else {
      return;
    };

    on_cleanup(move || {
      animation_frame_handle.cancel();
    });
  });

  let mut merged_attrs = vec![
    ("role", "tabpanel".into_attribute()),
    (
      "data-state",
      Signal::derive(move || {
        if is_selected.get() {
          "active"
        } else {
          "inactive"
        }
      })
      .into_attribute(),
    ),
    (
      "data-orientation",
      Signal::derive(move || match context.orientation.get() {
        Orientation::Horizontal => "horizontal",
        Orientation::Vertical => "vertical",
      })
      .into_attribute(),
    ),
    (
      "aria-labelledby",
      Signal::derive(move || trigger_id.get()).into_attribute(),
    ),
    (
      "hidden",
      Signal::derive(move || !is_present.get()).into_attribute(),
    ),
    (
      "id",
      Signal::derive(move || content_id.get()).into_attribute(),
    ),
    ("tabindex", 0.into_attribute()),
  ];
  merged_attrs.extend(attrs.clone().into_iter().map(|(name, attr)| {
    if name == "style" {
      let attr = Signal::derive(move || {
        format!(
          "{}{}",
          attr
            .as_nameless_value_string()
            .map(|value| format!("{}; ", value.to_string()))
            .unwrap_or_default(),
          is_mount_animation_prevented
            .get_value()
            .then_some("animation-duration: 0s")
            .unwrap_or_default(),
        )
      });

      (name, attr.into_attribute())
    } else {
      (name, attr)
    }
  }));

  view! {
    {move || is_present.get().then(|| {
      let attrs = merged_attrs.clone();
      let children = children.clone();

      view! {
        <Primitive
          element=html::div
          attrs=attrs
          node_ref=Some(node_ref)
        >
          {children()}
        </Primitive>
      }
    })}
  }
}
