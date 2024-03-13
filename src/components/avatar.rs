use leptos::*;
use wasm_bindgen::{closure::Closure, JsCast};

use crate::Attributes;

#[derive(Clone)]
pub struct AvatarContextValue {
  image_loading_status: Signal<ImageLoadingStatus>,
  on_image_loading_status_change: Callback<ImageLoadingStatus>,
}

#[component]
pub fn AvatarRoot(#[prop(attrs)] attrs: Attributes, children: Children) -> impl IntoView {
  let (image_loading_status, set_image_loading_status) = create_signal(ImageLoadingStatus::Idle);

  provide_context(AvatarContextValue {
    image_loading_status: image_loading_status.into_signal(),
    on_image_loading_status_change: Callback::new(set_image_loading_status),
  });

  view! {
    <span {..attrs}>
      {children()}
    </span>
  }
}

#[component]
pub fn AvatarImage(
  #[prop(optional)] on_loading_status_change: Option<Callback<ImageLoadingStatus>>,
  #[prop(attrs)] attrs: Attributes,
) -> impl IntoView {
  let context = use_context::<AvatarContextValue>()
    .expect("AvatarImage needs to be in an AvatarRoot component");

  let src = attrs
    .iter()
    .find(|item| item.0 == "src")
    .map(|item| {
      item
        .1
        .as_nameless_value_string()
        .map(|value| (move || value.to_string()).into_signal())
    })
    .flatten();

  let image_loading_status = use_image_loading_status(src);

  let handle_loading_status_change = move |status: ImageLoadingStatus| {
    if let Some(on_loading_status_change) = on_loading_status_change {
      on_loading_status_change(status.clone());
    }

    (context.on_image_loading_status_change)(status);
  };

  create_effect(move |_| {
    if image_loading_status.get() != ImageLoadingStatus::Idle {
      handle_loading_status_change(image_loading_status.get());
    }
  });

  view! {
    <Show when=move || image_loading_status.get() == ImageLoadingStatus::Loaded>
      <img {..attrs.clone()} />
    </Show>
  }
}

#[component]
pub fn AvatarFallback(
  #[prop(optional)] delay_ms: Option<i32>,
  #[prop(attrs)] attrs: Attributes,
  children: ChildrenFn,
) -> impl IntoView {
  let context = use_context::<AvatarContextValue>()
    .expect("AvatarFallback needs to be in an AvatarRoot component");
  let (can_render, set_can_render) = create_signal(delay_ms.is_none());

  create_effect(move |_| {
    if let Some(delay_ms) = delay_ms {
      let callback = Closure::<dyn Fn()>::new(move || set_can_render(true));
      let timer_id = window()
        .set_timeout_with_callback_and_timeout_and_arguments_0(
          callback.as_ref().unchecked_ref(),
          delay_ms,
        )
        .expect("could not create fallback timeout closure");

      callback.forget();

      on_cleanup(move || window().clear_timeout_with_handle(timer_id));
    }
  });

  view! {
    <Show when=move || can_render.get() && context.image_loading_status.get() != ImageLoadingStatus::Loaded>
      <span {..attrs.clone()}>{children()}</span>
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

fn use_image_loading_status(src: Option<Signal<String>>) -> Signal<ImageLoadingStatus> {
  let (loading_status, set_loading_status) = create_signal(ImageLoadingStatus::Idle);

  create_effect(move |_| {
    let Some(src) = src else {
      set_loading_status(ImageLoadingStatus::Error);
      return;
    };

    let is_mounted = StoredValue::new(true);
    let Ok(image) = document().create_element("img") else {
      set_loading_status(ImageLoadingStatus::Error);
      return;
    };

    let loaded_status_callback = Closure::<dyn FnMut(_)>::new(move |_: web_sys::Event| {
      if is_mounted.get_value() == false {
        return;
      }

      set_loading_status(ImageLoadingStatus::Loaded);
    });
    let error_status_callback = Closure::<dyn FnMut(_)>::new(move |_: web_sys::Event| {
      if is_mounted.get_value() == false {
        return;
      }

      set_loading_status(ImageLoadingStatus::Error);
    });

    image
      .add_event_listener_with_callback("load", loaded_status_callback.as_ref().unchecked_ref())
      .expect("could not add load event callback");

    image
      .add_event_listener_with_callback("error", error_status_callback.as_ref().unchecked_ref())
      .expect("could not add load event callback");

    image
      .set_attribute("src", &src.get())
      .expect("could not assign src attribute");

    loaded_status_callback.forget();
    error_status_callback.forget();

    on_cleanup(move || {
      is_mounted.set_value(false);
    });
  });

  loading_status.into_signal()
}
