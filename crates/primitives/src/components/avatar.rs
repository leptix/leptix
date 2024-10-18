use leptos::{
  html::{self, Img, Span},
  prelude::*,
};
use leptos_use::{use_timeout_fn, UseTimeoutFnReturn};
use wasm_bindgen::{closure::Closure, JsCast};

use crate::primitive::{Primitive, PrimitiveSelfClosing};

#[derive(Clone)]
pub struct AvatarContextValue {
  image_loading_status: Signal<ImageLoadingStatus>,
  on_image_loading_status_change: Callback<ImageLoadingStatus>,
}

#[component]
pub fn AvatarRoot(
  #[prop(optional)] node_ref: NodeRef<Span>,
  children: ChildrenFn,

  #[prop(optional, into)] as_child: MaybeProp<bool>,
) -> impl IntoView {
  let (image_loading_status, set_image_loading_status) = signal(ImageLoadingStatus::Idle);

  provide_context(AvatarContextValue {
    image_loading_status: Signal::derive(move || image_loading_status.get()),
    on_image_loading_status_change: Callback::new(move |status| {
      set_image_loading_status.set(status);
    }),
  });

  view! {
    <Primitive
      element={html::span}
      node_ref={node_ref}
      as_child={as_child}
    >
      {children()}
    </Primitive>
  }
}

#[component]
pub fn AvatarImage(
  #[prop(default=Callback::new(|_|{}), into)] on_loading_status_change: Callback<
    ImageLoadingStatus,
  >,

  #[prop(optional, into)] src: MaybeProp<String>,

  #[prop(optional)] node_ref: NodeRef<Img>,
  #[prop(optional)] children: Option<ChildrenFn>,

  #[prop(optional, into)] as_child: MaybeProp<bool>,
) -> impl IntoView {
  let context = use_context::<AvatarContextValue>()
    .expect("AvatarImage needs to be in an AvatarRoot component");

  let image_loading_status = use_image_loading_status(src);

  let handle_loading_status_change = move |status: ImageLoadingStatus| {
    on_loading_status_change.run(status.clone());
    context.on_image_loading_status_change.run(status);
  };

  Effect::new(move |_| {
    if image_loading_status.get() != ImageLoadingStatus::Idle {
      handle_loading_status_change(image_loading_status.get());
    }
  });

  let children = StoredValue::new(children);

  view! {
    <Show when=move || image_loading_status.get() == ImageLoadingStatus::Loaded>
      <PrimitiveSelfClosing
        element={html::img}
        node_ref={node_ref}
        as_child={as_child}
      >
        {children.with_value(|children| children.as_ref().map(|children| children()))}
      </PrimitiveSelfClosing>
    </Show>
  }
}

#[component]
pub fn AvatarFallback(
  #[prop(optional, into)] delay_ms: MaybeSignal<f64>,

  #[prop(optional)] node_ref: NodeRef<Span>,
  children: ChildrenFn,

  #[prop(optional, into)] as_child: MaybeProp<bool>,
) -> impl IntoView {
  let context = use_context::<AvatarContextValue>()
    .expect("AvatarFallback needs to be in an AvatarRoot component");
  let (can_render, set_can_render) = signal(false);

  Effect::new(move |_| {
    let UseTimeoutFnReturn { start, .. } = use_timeout_fn(
      move |_: ()| {
        set_can_render.set(true);
      },
      delay_ms.get(),
    );

    start(());
  });

  let children = StoredValue::new(children);

  view! {
    <Show when=move || can_render.get() && context.image_loading_status.get() != ImageLoadingStatus::Loaded>
      <Primitive
        element={html::span}
        node_ref={node_ref}
        as_child={as_child}
      >
        {children.with_value(|children| children())}
      </Primitive>
    </Show>
  }
}

#[derive(Clone, PartialEq, strum::Display)]
pub enum ImageLoadingStatus {
  Idle,
  Loaded,
  Loading,
  Error,
}

fn use_image_loading_status(src: MaybeProp<String>) -> Signal<ImageLoadingStatus> {
  let (loading_status, set_loading_status) = signal(ImageLoadingStatus::Idle);

  Effect::new(move |_| {
    let Some(src) = src.get() else {
      set_loading_status.set(ImageLoadingStatus::Error);
      return;
    };

    let is_mounted = StoredValue::new(true);
    let Ok(image) = document().create_element("img") else {
      set_loading_status.set(ImageLoadingStatus::Error);
      return;
    };

    let loaded_status_callback = Closure::<dyn FnMut(_)>::new(move |_: web_sys::Event| {
      if !is_mounted.get_value() {
        return;
      }

      set_loading_status.set(ImageLoadingStatus::Loaded);
    });
    let error_status_callback = Closure::<dyn FnMut(_)>::new(move |_: web_sys::Event| {
      if !is_mounted.get_value() {
        return;
      }

      set_loading_status.set(ImageLoadingStatus::Error);
    });

    image
      .add_event_listener_with_callback("load", loaded_status_callback.as_ref().unchecked_ref())
      .expect("could not add load event callback");

    image
      .add_event_listener_with_callback("error", error_status_callback.as_ref().unchecked_ref())
      .expect("could not add load event callback");

    image
      .set_attribute("src", &src)
      .expect("could not assign src attribute");

    loaded_status_callback.forget();
    error_status_callback.forget();

    on_cleanup(move || {
      is_mounted.set_value(false);
    });
  });

  loading_status.into()
}
