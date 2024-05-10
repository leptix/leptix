use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use leptos_use::{use_cookie, utils::FromToStringCodec};

use leptix_primitives::components::checkbox::{CheckboxIndicator, CheckboxRoot, CheckedState};

use crate::primitives::PrimitivesShowcase;

#[component]
pub fn App() -> impl IntoView {
  provide_meta_context();

  view! {
      <Stylesheet id="leptos" href="/pkg/ssr-with-actix-tailwind.css"/>
      <Title text="Leptix SSR"/>

      <Router>
          <main class="dark:bg-[#111113] p-4 flex flex-col gap-2 text-mauve11 dark:text-white">
              <Routes>
                  <Route path="" view=HomePage/>
                  <Route path="/*any" view=NotFound/>
              </Routes>
          </main>
      </Router>
  }
}

#[component]
fn Foo() -> impl IntoView {
  let foo_ref = NodeRef::<html::Div>::new();

  let is_form_control = Signal::derive(move || {
    if let Some(foo) = foo_ref.get()  {
      foo.closest("form").ok().flatten().is_some()
    } else {
      true
    }
  });

  logging::log!("{}", is_form_control.get());

  Effect::new(move |_| {
    logging::log!("{}", is_form_control.get());
  });

  view! {
    <div node_ref=foo_ref>"hello world"</div>
    <Show when=move || is_form_control.get()>
      <div>"hello world 2"</div>
    </Show>
  }
}

#[component]
fn CheckIcon() -> impl IntoView {
    view! {
      <svg width="15" height="15" viewBox="0 0 15 15" fill="none" xmlns="http://www.w3.org/2000/svg">
        <path
          d="M11.4669 3.72684C11.7558 3.91574 11.8369 4.30308 11.648 4.59198L7.39799 11.092C7.29783 11.2452 7.13556 11.3467 6.95402 11.3699C6.77247 11.3931 6.58989 11.3355 6.45446 11.2124L3.70446 8.71241C3.44905 8.48022 3.43023 8.08494 3.66242 7.82953C3.89461 7.57412 4.28989 7.55529 4.5453 7.78749L6.75292 9.79441L10.6018 3.90792C10.7907 3.61902 11.178 3.53795 11.4669 3.72684Z"
          fill="currentColor"
          fill-rule="evenodd"
          clip-rule="evenodd"
        ></path>
      </svg>
    }
}

#[component]
fn HomePage() -> impl IntoView {
  view! {
    //<form>
      <Foo />
    //</form>
    //<form>
      <Foo />
    //</form>
  }
}

#[component]
fn Auth() -> impl IntoView {
  let (auth_cookie, _) = use_cookie::<String, FromToStringCodec>("auth");
  let profile = create_resource(|| (), |_| async move { get_profile().await.ok() });
  let auth_signal = move || profile.get().or(Some(auth_cookie.get())).flatten();

  view! {

    {move || if auth_signal()
      .map(|auth_cookie| auth_cookie == "bob")
      .unwrap_or(false)
    {
      view! {
        <div class="flex gap-2 items-center">
            <span>"hello bob"</span>
            <button
                class="transition duration-75 rounded-md hover:bg-violet8 bg-violet9 active:bg-violet10 px-1.5 py-1 text-white text-sm font-semibold"
                on:click=move |_| {
                    spawn_local(async move {
                        _ = logout().await;
                        profile.refetch();
                    });
                }
            >
                "Logout"
            </button>
        </div>
      }
      .into_view()
    } else {
      view! {
        <button
          class="transition duration-75 rounded-md hover:bg-violet8 bg-violet9 active:bg-violet10 px-1.5 py-1 text-white text-sm font-semibold w-fit"
          on:click=move |_| {
            spawn_local(async move {
              _ = login().await;
              profile.refetch();
            });
          }
        >
          "Login"
        </button>
      }
      .into_view()
    }}
  }
}

#[server]
async fn get_profile() -> Result<String, ServerFnError> {
  use actix_web::HttpRequest;
  use leptos::server_fn::error::NoCustomError;
  use leptos_actix::extract;

  let foo = extract::<HttpRequest>()
    .await?
    .cookie("auth")
    .ok_or(ServerFnError::<NoCustomError>::ServerError(
      "unauthorized".to_string(),
    ))?
    .value()
    .to_string();

  Ok(foo)
}

#[server]
async fn login() -> Result<(), ServerFnError> {
  use actix_web::{
    cookie::{
      time::{Duration, OffsetDateTime, Time},
      Cookie,
    },
    http::header,
    http::header::HeaderValue,
  };
  use leptos_actix::ResponseOptions;
  use std::ops::Add;

  let mut cookie = Cookie::build("auth", "bob")
    .max_age(Duration::days(7))
    .expires(OffsetDateTime::now_utc() + Duration::days(7))
    .http_only(true)
    .path("/")
    .finish();

  if let Ok(cookie) = HeaderValue::from_str(&cookie.to_string()) {
    expect_context::<ResponseOptions>().insert_header(header::SET_COOKIE, cookie);
  }

  Ok(())
}

#[server]
async fn logout() -> Result<(), ServerFnError> {
  use actix_web::{
    cookie::{
      time::{Duration, OffsetDateTime, Time},
      Cookie,
    },
    http::header,
    http::header::HeaderValue,
  };
  use leptos_actix::ResponseOptions;
  use std::ops::Add;

  let mut cookie = Cookie::build("auth", "")
    .max_age(Duration::seconds(0))
    .expires(OffsetDateTime::now_utc() - Duration::seconds(1))
    .http_only(true)
    .path("/")
    .finish();

  if let Ok(cookie) = HeaderValue::from_str(&cookie.to_string()) {
    expect_context::<ResponseOptions>().insert_header(header::SET_COOKIE, cookie);
  }

  Ok(())
}

/// 404 - Not Found
#[component]
fn NotFound() -> impl IntoView {
  // set an HTTP status code 404
  // this is feature gated because it can only be done during
  // initial server-side rendering
  // if you navigate to the 404 page subsequently, the status
  // code will not be set because there is not a new HTTP request
  // to the server
  #[cfg(feature = "ssr")]
  {
    // this can be done inline because it's synchronous
    // if it were async, we'd use a server function
    let resp = expect_context::<leptos_actix::ResponseOptions>();
    resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
  }

  view! {
      <h1>"Not Found"</h1>
  }
}
