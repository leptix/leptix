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
  components::{presence::create_presence, primitive::Primitive},
  util::create_previous::create_previous,
  Attributes,
};

#[derive(Clone)]
struct RadioContextValue {
  checked: Signal<bool>,
  disabled: Signal<bool>,
}

#[component]
pub fn Radio(
  value: Signal<String>,
  #[prop(optional)] checked: Option<Signal<bool>>,
  #[prop(optional)] required: Option<Signal<bool>>,
  #[prop(optional)] on_check: Option<Callback<()>>,

  #[prop(optional)] disabled: Option<Signal<bool>>,
  #[prop(optional)] name: Option<Signal<Option<String>>>,
  #[prop(optional)] node_ref: NodeRef<AnyElement>,
  #[prop(attrs)] attrs: Attributes,
  children: Children,
) -> impl IntoView {
  let node_ref = NodeRef::<AnyElement>::new();
  let is_form_control = Signal::derive(move || {
    let Some(node) = node_ref.get() else {
      return true;
    };

    let Some(node_el) = node.dyn_ref::<HtmlButtonElement>() else {
      return true;
    };

    node_el.closest("form").ok().flatten().is_some()
  });

  let has_consumer_stopped_propagation = store_value(false);

  let mut merged_attrs = vec![
    ("type", "button".into_attribute()),
    ("role", "radio".into_attribute()),
    (
      "aria-checked",
      Signal::derive(move || {
        checked
          .map(|checked| checked.get())
          .unwrap_or(false)
          .to_string()
      })
      .into_attribute(),
    ),
    (
      "data-state",
      Signal::derive(move || {
        if checked.map(|checked| checked.get()).unwrap_or(false) {
          "checked"
        } else {
          "unchecked"
        }
      })
      .into_attribute(),
    ),
    (
      "data-disabled",
      Signal::derive(move || disabled.map(|disabled| disabled.get()).unwrap_or(false))
        .into_attribute(),
    ),
    (
      "disabled",
      Signal::derive(move || {
        disabled
          .map(|disabled| disabled.get().then_some(""))
          .flatten()
        // .unwrap_or(false)
        // .to_string()
      })
      .into_attribute(),
    ),
    (
      "value",
      Signal::derive(move || value.get()).into_attribute(),
    ),
  ];

  merged_attrs.extend(attrs.into_iter());

  provide_context(RadioContextValue {
    checked: Signal::derive(move || checked.map(|checked| checked.get()).unwrap_or(false)),
    disabled: Signal::derive(move || disabled.map(|disabled| disabled.get()).unwrap_or(false)),
  });

  view! {
    <Primitive
      element=html::button
      attrs=merged_attrs
      node_ref=Some(node_ref)
      on:click=move |ev: MouseEvent| {
        if checked.map(|checked| checked.get()).unwrap_or(false) == false {
          if let Some(on_check) = on_check {
            on_check(())
          }
        }

        if is_form_control.get() {
          // has_consumer_stopped_propagation.set_value(ev.is_propagation_stopped());

          if has_consumer_stopped_propagation.get_value() == false {
            ev.stop_propagation();
          }
        }
      }
    >
      {children()}
      {move || is_form_control.get().then(|| view! {
        <BubbleInput
          checked=Signal::derive(move || checked.map(|checked| checked.get()).unwrap_or(false))
          bubbles=Signal::derive(move || !has_consumer_stopped_propagation.get_value())
          name=Signal::derive(move || name.map(|name| name.get()).flatten())
          value=Signal::derive(move || value.get())
          required=Signal::derive(move || required.map(|required| required.get()).unwrap_or(false))
          disabled=Signal::derive(move || disabled.map(|disabled| disabled.get()).unwrap_or(false))
          control=node_ref
        />
      })}
    </Primitive>
  }
}

#[component]
pub fn RadioIndicator(
  #[prop(optional)] force_mount: Option<Signal<bool>>,
  #[prop(optional)] node_ref: NodeRef<AnyElement>,
  #[prop(attrs)] attrs: Attributes,
  children: ChildrenFn,
) -> impl IntoView {
  let context =
    use_context::<RadioContextValue>().expect("RadioIndicator must be used in a Radio component");

  let is_present = Signal::derive(move || {
    force_mount
      .map(|force_mount| force_mount.get())
      .unwrap_or(false)
      || context.checked.get()
  });

  // let presence = create_presence(is_present);

  view! {
    <>
      {move || is_present.get().then(|| {
        let children = children.clone();
        let mut merged_attrs = attrs.clone();

        merged_attrs.extend(
          [
            (
              "data-state",
              Signal::derive(move || if context.checked.get() { "checked" } else { "unchecked"} ).into_attribute()
            ),
            (
              "data-disabled",
              Signal::derive(move || context.disabled.get()).into_attribute()
            ),
          ]
          .into_iter()
        );

        view! {
          <Primitive
            element=html::span
            node_ref=Some(node_ref)
            attrs=merged_attrs
          >
            {children()}
          </Primitive>
        }
      })}
    </>
  }
}

#[component]
fn BubbleInput(
  checked: Signal<bool>,
  bubbles: Signal<bool>,
  name: Signal<Option<String>>,
  value: Signal<String>,
  required: Signal<bool>,
  disabled: Signal<bool>,
  control: NodeRef<AnyElement>,
  #[prop(attrs)] attrs: Attributes,
) -> impl IntoView {
  let node_ref = NodeRef::<Input>::new();
  let prev_checked = create_previous(Signal::derive(move || checked.get()));
  let UseElementSizeReturn { width, height } = use_element_size(control);

  create_effect(move |_| {
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
      name=Signal::derive(move || name.get()).into_attribute()
      value=Signal::derive(move || value.get()).into_attribute()
      required=Signal::derive(move || required.get()).into_attribute()
      disabled=Signal::derive(move || disabled.get()).into_attribute()
      checked=Signal::derive(move || checked.get()).into_attribute()
      tabindex=(-1).into_attribute()
      node_ref=node_ref
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
