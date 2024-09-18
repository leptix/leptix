use leptos::{
  html::{CreateElement, ElementType, HtmlElement},
  prelude::*,
};

use wasm_bindgen::JsCast;

#[component]
pub fn Primitive<El, NewChild>(
  element: fn() -> HtmlElement<El, (), (), Dom>,
  children: TypedChildrenFn<NewChild>,
  #[prop(optional)] as_child: MaybeProp<bool>,
  #[prop(optional)] node_ref: NodeRef<El>,
) -> impl IntoView
where
  HtmlElement<El, (), (), Dom>: ElementChild<Dom, View<NewChild>>,
  <HtmlElement<El, (), (), Dom> as ElementChild<Dom, View<NewChild>>>::Output: IntoView,
  El: ElementType + CreateElement<Dom> + 'static,
  <El as ElementType>::Output: JsCast,
  NewChild: IntoView + Send + 'static,
{
  let children = StoredValue::new(children.into_inner());
  use leptos::tachys::html::node_ref::node_ref as r#ref;

  view! {
    <Show
      when=move || as_child.get().unwrap_or_default()
      fallback=move || {
        element()
          .child(children.with_value(|children| children()))
          .add_any_attr(r#ref(node_ref))
      }
    >
      {children.with_value(|children| children())}
    </Show>
  }
}

#[component]
pub fn PrimitiveSelfClosing<El, NewChild>(
  element: fn() -> HtmlElement<El, (), (), Dom>,
  children: TypedChildrenFn<NewChild>,
  #[prop(optional)] as_child: MaybeProp<bool>,
  #[prop(optional)] node_ref: NodeRef<El>,
) -> impl IntoView
where
  El: ElementType + CreateElement<Dom> + 'static,
  <El as ElementType>::Output: JsCast,
  NewChild: IntoView + Send + 'static,
{
  let children = StoredValue::new(children.into_inner());
  use leptos::tachys::html::node_ref::node_ref as r#ref;

  view! {
    <Show
      when=move || as_child.get().unwrap_or_default()
      fallback=move || {
        element()
          .add_any_attr(r#ref(node_ref))
      }
    >
      {children.with_value(|children| children())}
    </Show>
  }
}
