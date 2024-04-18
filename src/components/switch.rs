use leptos::{
  html::{AnyElement, Input},
  *,
};
use leptos_use::{use_element_size, UseElementSizeReturn};
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{
  js_sys::{Array, Function, JsString, Object, Reflect},
  Event, EventInit, HtmlButtonElement, MouseEvent,
};

use crate::{
  components::primitive::Primitive,
  util::{
    create_controllable_signal::{create_controllable_signal, CreateControllableSignalProps},
    create_previous::create_previous,
  },
  Attributes,
};

#[derive(Clone)]
struct SwitchContextValue {
  checked: Signal<bool>,
  disabled: Signal<bool>,
}

#[component]
pub fn SwitchRoot(
  #[prop(optional)] checked: Option<MaybeSignal<bool>>,
  #[prop(optional)] value: Option<MaybeSignal<String>>,
  #[prop(optional)] name: Option<MaybeSignal<String>>,
  #[prop(optional)] disabled: Option<MaybeSignal<bool>>,
  #[prop(optional)] default_checked: Option<MaybeSignal<bool>>,
  #[prop(optional)] required: Option<MaybeSignal<bool>>,
  #[prop(optional)] on_checked_change: Option<Callback<bool>>,
  #[prop(optional)] on_click: Option<Callback<MouseEvent>>,

  #[prop(attrs)] attrs: Attributes,
  #[prop(optional)] node_ref: NodeRef<AnyElement>,
  children: Children,
) -> impl IntoView {
  let node_ref = NodeRef::<AnyElement>::new();
  let is_form_control = Signal::derive(move || {
    if let Some(node) = node_ref.get() {
      node.closest("form").ok().flatten().is_some()
    } else {
      true
    }
  });

  let has_consumer_stopped_propagation = StoredValue::new(false);

  let (checked, set_checked) = create_controllable_signal(CreateControllableSignalProps {
    value: Signal::derive(move || checked.map(|checked| checked.get())),
    default_value: Signal::derive(move || {
      default_checked.map(|default_checked| default_checked.get())
    }),
    on_change: Callback::new(move |value| {
      if let Some(on_checked_change) = on_checked_change {
        on_checked_change(value);
      }
    }),
  });

  provide_context(SwitchContextValue {
    checked: Signal::derive(move || checked.get().unwrap_or(false)),
    disabled: Signal::derive(move || disabled.map(|disabled| disabled.get()).unwrap_or(false)),
  });

  let attr_value = value.clone();

  let mut merged_attrs = attrs.clone();
  merged_attrs.extend(
    [
      ("type", "button".into_attribute()),
      ("role", "switch".into_attribute()),
      (
        "aria-checked",
        Signal::derive(move || checked.get()).into_attribute(),
      ),
      (
        "aria-required",
        Signal::derive(move || required.map(|required| required.get())).into_attribute(),
      ),
      (
        "data-state",
        Signal::derive(move || {
          if checked.get().unwrap_or(false) {
            "checked"
          } else {
            "unchecked"
          }
        })
        .into_attribute(),
      ),
      (
        "data-disabled",
        Signal::derive(move || disabled.map(|disabled| disabled.get())).into_attribute(),
      ),
      (
        "value",
        Signal::derive(move || attr_value.as_ref().map(|value| value.get())).into_attribute(),
      ),
    ]
    .into_iter(),
  );

  let inner_name = name.clone();
  let inner_value = value.clone();

  view! {
    <Primitive
      element=html::button
      attrs=merged_attrs
      node_ref=node_ref
      on:click=move |ev: MouseEvent| {
        if let Some(on_click) = on_click {
          on_click(ev.clone());
        }

        set_checked.update(|checked| *checked = Some(!checked.unwrap_or(false)));

        if is_form_control.get() {
          // has_consumer_stopped_propagation.set_value(ev.is_propagation_stopped());

          if has_consumer_stopped_propagation.get_value() == false {
            ev.stop_propagation();
          }
        }
      }
    >
      {children()}
      {move || is_form_control.get().then(|| {
        let inner_name = inner_name.clone();
        let inner_value = inner_value.clone();

        view! {
          <BubbleInput
            checked=Signal::derive(move || checked.get().unwrap_or(false))
            bubbles=Signal::derive(move || !has_consumer_stopped_propagation.get_value())
            name=Signal::derive(move || inner_name.as_ref().map(|name| name.get()))
            value=Signal::derive(move || inner_value.as_ref().map(|name| name.get()).unwrap_or("on".to_string()))
            disabled=Signal::derive(move || disabled.map(|disabled| disabled.get()).unwrap_or(false))
            required=Signal::derive(move || required.map(|required| required.get()).unwrap_or(false))
            control=node_ref
          />
        }
      })}
    </Primitive>
  }
}

#[component]
pub fn SwitchThumb(
  #[prop(attrs)] attrs: Attributes,
  #[prop(optional)] node_ref: NodeRef<AnyElement>,
) -> impl IntoView {
  let context = use_context::<SwitchContextValue>()
    .expect("SwitchThumb must be used in a SwitchRoot component");

  let mut merged_attrs = attrs.clone();
  merged_attrs.extend(
    [
      (
        "data-state",
        Signal::derive(move || {
          if context.checked.get() {
            "checked"
          } else {
            "unchecked"
          }
        })
        .into_attribute(),
      ),
      (
        "data-disabled",
        Signal::derive(move || context.disabled.get().then_some("")).into_attribute(),
      ),
    ]
    .into_iter(),
  );

  view! {
    <Primitive
      element=html::span
      node_ref=node_ref
      attrs=merged_attrs
    >
      {().into_view()}
    </Primitive>
  }
}

#[component]
pub fn BubbleInput(
  checked: Signal<bool>,
  bubbles: Signal<bool>,
  name: Signal<Option<String>>,
  value: Signal<String>,
  disabled: Signal<bool>,
  required: Signal<bool>,
  control: NodeRef<AnyElement>,
  #[prop(attrs)] attrs: Attributes,
) -> impl IntoView {
  let node_ref = NodeRef::<Input>::new();
  let prev_checked = create_previous(Signal::derive(move || checked.get()));
  let UseElementSizeReturn { width, height } = use_element_size(control);

  Effect::new(move |_| {
    (|| {
      let input = node_ref.get()?;
      let input_el = window().get("HTMLInputElement")?;
      let input_proto = Reflect::get(&input_el, &JsString::from("prototype"))
        .ok()?
        .dyn_into::<Object>()
        .ok()?;

      let input_descriptor_set = Reflect::get(
        &Object::get_own_property_descriptor(&input_proto, &JsString::from("checked")),
        &JsString::from("set"),
      )
      .ok()?
      .dyn_into::<Function>()
      .ok()?;

      if prev_checked.get() != checked.get() {
        let mut ev_options = EventInit::new();
        ev_options.bubbles(bubbles.get());

        let ev = Event::new_with_event_init_dict("click", &ev_options).ok()?;

        _ = Reflect::apply(
          &input_descriptor_set,
          &input,
          &Array::from_iter([JsValue::from_bool(checked.get())].into_iter()),
        );

        _ = input.dispatch_event(&ev);
      }

      Some(())
    })();
  });

  view! {
    <input
      type="checkbox"
      aria-hidden
      checked=Signal::derive(move || checked.get()).into_attribute()
      tabindex=(-1).into_attribute()
      node_ref=node_ref
      name=Signal::derive(move || name.get()).into_attribute()
      value=Signal::derive(move || value.get()).into_attribute()
      disabled=Signal::derive(move || disabled.get()).into_attribute()
      required=Signal::derive(move || required.get()).into_attribute()
      style:position="absolute"
      style:pointer-events="none"
      style:opacity="0"
      style:margin="0"
      style:width=move || width.get()
      style:height=move || height.get()
      {..attrs}
    />
  }
}
