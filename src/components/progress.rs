use leptos::{html::AnyElement, *};

use crate::{components::primitive::Primitive, Attributes};

const DEFAULT_MAX: u32 = 100;

#[derive(Clone)]
struct ProgressContextValue {
  value: Option<Signal<u32>>,
  max: Signal<u32>,
}

#[component]
pub fn ProgressRoot(
  #[prop(optional)] value: Option<Signal<u32>>,
  #[prop(optional)] max: Option<Signal<u32>>,
  #[prop(optional)] get_value_label: Option<Callback<(u32, u32), String>>,
  #[prop(attrs)] attrs: Attributes,
  #[prop(optional)] node_ref: NodeRef<AnyElement>,
  children: Children,
) -> impl IntoView {
  let max = max.unwrap_or((move || DEFAULT_MAX).into_signal());
  let value = value.map(|value| (move || value.get() % (max.get() + 1)).into_signal());

  let value_label = value
    .map(|value| {
      get_value_label
        .map(|get_value_label| (move || get_value_label((value.get(), max.get()))).into_signal())
    })
    .flatten();

  provide_context(ProgressContextValue { value, max });

  let mut merged_attrs = attrs.clone();

  merged_attrs.extend(
    [
      ("role", "progressbar".into_attribute()),
      (
        "aria-valuemax",
        Signal::derive(move || max.get()).into_attribute(),
      ),
      ("aria-valuemin", 0.into_attribute()),
      (
        "aria-valuenow",
        Signal::derive(move || value.map(|value| value.get())).into_attribute(),
      ),
      (
        "aria-valuetext",
        Signal::derive(move || value_label.map(|value_label| value_label.get())).into_attribute(),
      ),
      (
        "data-state",
        Signal::derive(move || {
          value
            .map(|value| {
              if value.get() == max.get() {
                "complete"
              } else {
                "loading"
              }
            })
            .unwrap_or("indeterminate")
        })
        .into_attribute(),
      ),
      (
        "data-value",
        Signal::derive(move || value.map(|value| value.get())).into_attribute(),
      ),
      (
        "data-max",
        Signal::derive(move || max.get()).into_attribute(),
      ),
    ]
    .into_iter(),
  );

  view! {
    <Primitive
      element=html::div
      node_ref=Some(node_ref)
      attrs=merged_attrs
    >
      {children()}
    </Primitive>
  }
}

#[component]
pub fn ProgressIndicator(
  #[prop(optional)] node_ref: NodeRef<AnyElement>,
  #[prop(attrs)] attrs: Attributes,
) -> impl IntoView {
  let context = use_context::<ProgressContextValue>()
    .expect("ProgressIndicator needs to be in a Progress component");

  let mut merged_attrs = attrs.clone();

  merged_attrs.extend(
    [
      (
        "data-state",
        Signal::derive(move || {
          context
            .value
            .map(|value| {
              if value.get() == context.max.get() {
                "complete"
              } else {
                "loading"
              }
            })
            .unwrap_or("indeterminate")
        })
        .into_attribute(),
      ),
      (
        "data-value",
        Signal::derive(move || context.value.map(|value| value.get())).into_attribute(),
      ),
      (
        "data-max",
        Signal::derive(move || context.max.get()).into_attribute(),
      ),
    ]
    .into_iter(),
  );

  view! {
    <Primitive
      element=html::div
      node_ref=Some(node_ref)
      attrs=merged_attrs
    >
      {().into_view()}
    </Primitive>
  }
}
