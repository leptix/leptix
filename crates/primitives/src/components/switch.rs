use leptos::{
  html::{AnyElement, Input},
  *,
};
use leptos_use::{use_element_size, UseElementSizeReturn};
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{
  js_sys::{Array, Function, JsString, Object, Reflect},
  Event, EventInit, MouseEvent,
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
  #[prop(optional, into)] checked: Option<MaybeSignal<bool>>,
  #[prop(optional, into)] default_checked: Option<MaybeSignal<bool>>,
  #[prop(optional, into)] value: Option<MaybeSignal<String>>,
  #[prop(optional, into)] name: Option<MaybeSignal<String>>,
  #[prop(optional, into)] disabled: MaybeSignal<bool>,
  #[prop(optional, into)] required: MaybeSignal<bool>,
  #[prop(default=Callback::new(|_:bool|{}), into)] on_checked_change: Callback<bool>,
  #[prop(default=Callback::new(|_:MouseEvent|{}), into)] on_click: Callback<MouseEvent>,

  #[prop(attrs)] attrs: Attributes,
  #[prop(optional)] node_ref: NodeRef<AnyElement>,
  children: Children,
) -> impl IntoView {
  let node_ref = NodeRef::<AnyElement>::new();
  let (is_form_control, set_is_form_control) = create_signal(true);

  let has_consumer_stopped_propagation = StoredValue::new(false);

  let (checked, set_checked) = create_controllable_signal(CreateControllableSignalProps {
    value: Signal::derive(move || checked.map(|checked| checked.get())),
    default_value: Signal::derive(move || {
      default_checked.map(|default_checked| default_checked.get())
    }),
    on_change: on_checked_change,
  });

  Effect::new(move |_| {
    set_is_form_control.set(if let Some(foo) = node_ref.get() {
      foo.closest("form").ok().flatten().is_some()
    } else {
      true
    });
  });

  provide_context(SwitchContextValue {
    checked: Signal::derive(move || checked.get().unwrap_or(false)),
    disabled: Signal::derive(move || disabled.get()),
  });

  let attr_value = value.clone();

  let mut merged_attrs = attrs.clone();
  merged_attrs.extend([
    ("type", "button".into_attribute()),
    ("role", "switch".into_attribute()),
    (
      "aria-checked",
      Signal::derive(move || checked.get()).into_attribute(),
    ),
    ("aria-required", required.into_attribute()),
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
    ("data-disabled", disabled.into_attribute()),
    (
      "value",
      Signal::derive(move || attr_value.as_ref().map(|value| value.get())).into_attribute(),
    ),
  ]);

  let inner_name = name.clone();
  let inner_value = value.clone();

  view! {
    <Primitive
      element=html::button
      attrs=merged_attrs
      node_ref=node_ref
      on:click=move |ev: MouseEvent| {
          on_click.call(ev.clone());

        set_checked.update(|checked| *checked = Some(!checked.unwrap_or(false)));

        if is_form_control.get() {
          // has_consumer_stopped_propagation.set_value(ev.is_propagation_stopped());

          if !has_consumer_stopped_propagation.get_value() {
            ev.stop_propagation();
          }
        }
      }
    >
      {children()}

      <Show when=move || is_form_control.get()>
        <BubbleInput
            checked=Signal::derive(move || checked.get().unwrap_or(false))
            bubbles=Signal::derive(move || !has_consumer_stopped_propagation.get_value())
            name=name.clone().map(|name| Signal::derive(move || name.get()))
            value=value.clone().map(|value| Signal::derive(move || value.get())).unwrap_or(Signal::derive(|| "on".to_string()))
            disabled=Signal::derive(move || disabled.get())
            required=Signal::derive(move || required.get())
            control=node_ref
        />
      </Show>
    </Primitive>
  }
}

#[component]
pub fn SwitchThumb(
  #[prop(attrs)] attrs: Attributes,
  #[prop(optional)] node_ref: NodeRef<AnyElement>,
) -> impl IntoView {
  let SwitchContextValue { checked, disabled } =
    use_context().expect("SwitchThumb must be used in a SwitchRoot component");

  let mut merged_attrs = attrs.clone();
  merged_attrs.extend([
    (
      "data-state",
      Signal::derive(move || {
        if checked.get() {
          "checked"
        } else {
          "unchecked"
        }
      })
      .into_attribute(),
    ),
    (
      "data-disabled",
      Signal::derive(move || disabled.get().then_some("")).into_attribute(),
    ),
  ]);

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
  name: Option<Signal<String>>,
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
          &Array::from_iter([JsValue::from_bool(checked.get())]),
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
      name=name.into_attribute()
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
