use leptos::{
  html::{self, Div},
  prelude::*,
};

use crate::primitive::Primitive;

const DEFAULT_MAX: f64 = 100.0;

#[derive(Clone)]
struct ProgressContextValue {
  value: Signal<Option<f64>>,
  max: Signal<f64>,
}

#[component]
pub fn ProgressRoot(
  #[prop(optional, into)] value: MaybeProp<f64>,
  #[prop(default=100.0f64.into(), into)] max: MaybeSignal<f64>,

  #[prop(optional)] get_value_label: Option<Callback<(f64, f64), String>>,

  #[prop(optional)] node_ref: NodeRef<Div>,
  children: ChildrenFn,

  #[prop(optional, into)] as_child: MaybeProp<bool>,
) -> impl IntoView {
  let max = Signal::derive(move || {
    let max = max.get();

    if !max.is_nan() && max > 0.0 {
      max
    } else {
      DEFAULT_MAX
    }
  });

  let value = Signal::derive(move || {
    let max = max.get();

    value
      .get()
      .and_then(|value| (!value.is_nan() && value <= max && value >= 0.0).then_some(value))
  });

  let get_value_label = get_value_label.unwrap_or(Callback::new(|(value, max): (f64, f64)| {
    format!("{}%", (value / max).round() * 100.0)
  }));

  let value_label = Signal::derive(move || {
    value
      .get()
      .map(|value| get_value_label.run((value, max.get())))
  });

  provide_context(ProgressContextValue {
    value,
    max: Signal::derive(move || max.get()),
  });

  view! {
    <Primitive
      element={html::div}
      node_ref={node_ref}
      as_child={as_child}
      attr:data-state=move || {
        value
        .get()
        .map(|value| {
          if value == max.get() {
            "complete"
          } else {
            "loading"
          }
        })
        .unwrap_or("indeterminate")
      }
      {..}
      data-value={value}
      data-max={max}
      aria-valuemax={max}
      aria-valuemin={0}
      aria-valuenow={value}
      aria-valuetext={value_label}
      role="progressbar"
    >
      {children()}
    </Primitive>
  }
}

#[component]
pub fn ProgressIndicator(
  #[prop(optional)] node_ref: NodeRef<Div>,
  #[prop(optional)] children: Option<ChildrenFn>,

  #[prop(optional, into)] as_child: MaybeProp<bool>,
) -> impl IntoView {
  let ProgressContextValue { max, value } =
    use_context().expect("ProgressIndicator needs to be in a Progress component");

  let children = StoredValue::new(children);

  view! {
    <Primitive
      element={html::div}
      node_ref={node_ref}
      as_child={as_child}
      attr:data-state=move || {
        value
        .get()
        .map(|value| {
          if value == max.get() {
            "complete"
          } else {
            "loading"
          }
        })
        .unwrap_or("indeterminate")
      }
      attr:data-value=value
      attr:data-max=max
    >
      {children.with_value(|children| children.as_ref().map(|children| children()))}
    </Primitive>
  }
}
