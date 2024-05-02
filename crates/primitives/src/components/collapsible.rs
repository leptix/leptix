use leptos::{html::AnyElement, *};
use wasm_bindgen::JsValue;
use web_sys::{js_sys::Object, CssStyleDeclaration, MouseEvent};

use crate::{
  components::primitive::Primitive,
  util::{
    create_controllable_signal::{create_controllable_signal, CreateControllableSignalProps},
    create_id::create_id,
  },
  Attributes,
};

use super::presence::create_presence;

#[derive(Clone)]
struct CollapsibleContextValue {
  content_id: Signal<String>,
  disabled: Signal<Option<bool>>,
  open: Signal<bool>,
  on_open_toggle: Callback<()>,
}

#[component]
pub fn CollapsibleRoot(
  #[prop(optional)] open: Option<MaybeSignal<bool>>,
  #[prop(optional)] default_open: Option<MaybeSignal<bool>>,
  #[prop(optional)] disabled: Option<MaybeSignal<bool>>,
  #[prop(optional)] on_open_change: Option<Callback<bool>>,
  #[prop(optional)] on_click: Option<Callback<MouseEvent>>,
  #[prop(optional)] as_child: Option<bool>,
  #[prop(optional)] node_ref: NodeRef<AnyElement>,
  #[prop(attrs)] attrs: Attributes,
  children: Children,
) -> impl IntoView {
  let (open, set_open) = create_controllable_signal(CreateControllableSignalProps {
    value: Signal::derive(move || open.map(|open| open.get())),
    default_value: Signal::derive(move || default_open.map(|default_open| default_open.get())),
    on_change: Callback::new(move |value| {
      if let Some(on_open_change) = on_open_change {
        on_open_change.call(value);
      }
    }),
  });

  provide_context(CollapsibleContextValue {
    open: Signal::derive(move || open.get().unwrap_or(false)),
    disabled: Signal::derive(move || disabled.map(|disabled| disabled.get())),
    content_id: create_id(),
    on_open_toggle: Callback::new(move |_| {
      set_open.update(|open| *open = Some(!open.unwrap_or(false)))
    }),
  });

  let mut merged_attrs = vec![
    (
      "data-state",
      (move || {
        if open.get().unwrap_or(false) {
          "open"
        } else {
          "closed"
        }
      })
      .into_attribute(),
    ),
    (
      "data-disabled",
      (move || disabled.map(|disabled| disabled.get()).unwrap_or(false)).into_attribute(),
    ),
  ];

  merged_attrs.extend(attrs);

  view! {
    <Primitive
      element=html::div
      node_ref=node_ref
      as_child=as_child
      attrs=merged_attrs
    >
      {children()}
    </Primitive>
  }
}

#[component]
pub fn CollapsibleTrigger(
  #[prop(optional)] as_child: Option<bool>,
  #[prop(optional)] node_ref: NodeRef<AnyElement>,
  #[prop(optional)] on_click: Option<Callback<MouseEvent>>,
  #[prop(attrs)] attrs: Attributes,
  children: Children,
) -> impl IntoView {
  let CollapsibleContextValue {
    content_id,
    disabled,
    open,
    on_open_toggle,
  } = use_context::<CollapsibleContextValue>()
    .expect("CollapsibleTrigger must be used in a CollapsibleRoot component");

  let mut merged_attrs = vec![
    ("aria-controls", (move || content_id.get()).into_attribute()),
    ("aria-expanded", (move || open.get()).into_attribute()),
    (
      "data-state",
      (move || if open.get() { "open" } else { "closed" }).into_attribute(),
    ),
    (
      "data-disabled",
      (move || disabled.get().unwrap_or(false)).into_attribute(),
    ),
    (
      "disabled",
      (move || disabled.get().unwrap_or(false)).into_attribute(),
    ),
  ];

  merged_attrs.extend(attrs);

  view! {
    <Primitive
      element=html::button
      attrs=merged_attrs
      node_ref=node_ref
      as_child=as_child
      on:click=move |ev: MouseEvent| {
        if let Some(on_click) = on_click {
          on_click.call(ev);
        }

        on_open_toggle.call(());
      }
    >
      {children()}
    </Primitive>
  }
}

#[component]
pub fn CollapsibleContent(
  #[prop(optional)] force_mount: Option<MaybeSignal<bool>>,
  #[prop(optional)] as_child: Option<bool>,
  #[prop(optional)] node_ref: NodeRef<AnyElement>,
  #[prop(attrs)] attrs: Attributes,
  children: ChildrenFn,
) -> impl IntoView {
  let CollapsibleContextValue { open, .. } = use_context::<CollapsibleContextValue>()
    .expect("CollapsibleContent must be used in a CollapsibleRoot component");

  let is_present = Signal::derive(move || {
    open.get()
      || force_mount
        .map(|force_mount| force_mount.get())
        .unwrap_or(false)
  });

  let presence = create_presence(is_present, node_ref);
  let children = StoredValue::new(children);

  view! {
    <Show when=move || presence.get()>
        <CollapsibleContentImpl
            as_child=as_child
            attrs=attrs.clone()
            node_ref=node_ref
            is_present=presence
        >
            {children.with_value(|children| children())}
        </CollapsibleContentImpl>
    </Show>
  }
}

#[component]
fn CollapsibleContentImpl(
  as_child: Option<bool>,
  #[prop(attrs)] attrs: Attributes,
  node_ref: NodeRef<AnyElement>,
  is_present: Signal<bool>,
  children: ChildrenFn,
) -> impl IntoView {
  let CollapsibleContextValue {
    content_id,
    disabled,
    open,
    ..
  } = use_context::<CollapsibleContextValue>()
    .expect("CollapsibleContentImpl must be used in a CollapsibleRoot component");

  let is_open = Signal::derive(move || open.get() || is_present.get());
  let is_mount_animation_prevented = StoredValue::new(is_open.get());

  let original_styles = StoredValue::<Option<CssStyleDeclaration>>::new(None);

  Effect::new(move |_| {
    let Ok(animation_frame) = request_animation_frame_with_handle(move || {
      is_mount_animation_prevented.set_value(false);
    }) else {
      return;
    };

    on_cleanup(move || {
      animation_frame.cancel();
    });
  });

  let rect_size = Signal::derive(move || {
    let mut node = node_ref.get()?;
    let node_style = window().get_computed_style(&node).ok()?;

    if original_styles.get_value().is_none() {
      let new_styles = CssStyleDeclaration::from(JsValue::from(Object::new()));

      if let Some(node_style) = node_style {
        if let Ok(transition_duration) = node_style.get_property_value("transition-duration") {
          _ = new_styles.set_property("transition-duration", &transition_duration);
        }

        if let Ok(animation_name) = node_style.get_property_value("animation-name") {
          _ = new_styles.set_property("animation-name", &animation_name);
        }
      }

      original_styles.set_value(Some(new_styles));
    }

    node = node
      .style("transition-duration", "0s")
      .style("animation-name", "none");

    let rect = node.get_bounding_client_rect();

    if !is_mount_animation_prevented.get_value() {
      _ = node
        .style(
          "transition-duration",
          original_styles
            .get_value()
            .and_then(|styles| styles.get_property_value("transition-duration").ok()),
        )
        .style(
          "animation-name",
          original_styles
            .get_value()
            .and_then(|styles| styles.get_property_value("animation-name").ok()),
        );
    }

    logging::log!("{} {}", rect.width(), rect.height());

    Some((rect.width(), rect.height()))
  });

  let present_state = Signal::derive(move || {
    rect_size
      .get()
      .map(|_| is_present.get())
      .unwrap_or(is_present.get())
  });

  Effect::new(move |_| {
    let Some(node) = node_ref.get() else {
      return;
    };

    let Some((width, height)) = rect_size.get() else {
      return;
    };

    _ = node
      .style(
        "--primitive-collapsible-content-width",
        format!("{width}px"),
      )
      .style(
        "--primitive-collapsible-content-height",
        format!("{height}px"),
      );
  });

  let mut merged_attrs = vec![
    (
      "data-state",
      (move || {
        if open.get() {
          "open"
        } else {
          "closed"
        }
      })
      .into_attribute(),
    ),
    (
      "data-disabled",
      (move || disabled.get().unwrap_or(false)).into_attribute(),
    ),
    ("id", (move || content_id.get()).into_attribute()),
    (
      "hidden",
      (move || !(is_open.get() || present_state.get())).into_attribute(),
    ),
  ];

  merged_attrs.extend(attrs);

  view! {
    <Primitive
      element=html::div
      attrs=merged_attrs
      as_child=as_child
      node_ref=node_ref
    >
      <Show when=move || is_open.get()>
        {children()}
      </Show>
    </Primitive>
  }
}
