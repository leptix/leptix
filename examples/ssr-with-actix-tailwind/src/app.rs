use leptos::{task::spawn_local, prelude::*};
use leptos_meta::*;
use leptos_router::{*, components::{Route, Routes, Router}};
use leptos_use::use_cookie;
use codee::string::FromToStringCodec;

use crate::primitives::PrimitivesShowcase;

#[derive(Clone)]
pub(crate) struct DarkThemeContext {
  pub(crate) dark_theme: RwSignal<bool>,
}

pub fn shell(options: LeptosOptions) -> impl IntoView {
  let (dark, _) = use_cookie::<String, FromToStringCodec>("dark");
  let dark_theme = RwSignal::new(dark.get_untracked().is_some());

  provide_context(DarkThemeContext {
    dark_theme
  });

  view! {
    <!DOCTYPE html>
    <html lang="en" class=move || if dark.get().is_some() { "dark" } else { "" }>
      <head>
        <meta charset="utf-8"/>
        <link rel="stylesheet" id="leptos" href="/pkg/ssr-with-actix-tailwind.css"/>
        <meta name="viewport" content="width=device-width, initial-scale=1"/>
        <AutoReload options=options.clone() />
        <HydrationScripts options/>
        <MetaTags/>
      </head>
      <body>
        <App />
      </body>
    </html>
  }
}

#[component]
pub fn App() -> impl IntoView {
  provide_meta_context();

  view! {
    <Title text="Leptix SSR"/>

    <Router>
      <main class="dark:bg-[#111113] p-4 flex flex-col gap-2 text-mauve11 dark:text-white">
      <Routes fallback=|| "Not found">
          <Route path=path!("") view=HomePage/>
          <Route path=path!("/*any") view=NotFound/>
        </Routes>
      </main>
    </Router>
  }
}

#[component]
fn HomePage() -> impl IntoView {
  view! {
    <Auth/>
    <PrimitivesShowcase/>
  }
}

#[component]
fn Auth() -> impl IntoView {
  let refetch_flag = RwSignal::new(0);

  let (auth_cookie, _) = use_cookie::<String, FromToStringCodec>("auth");
  let profile = LocalResource::new(|| async { get_profile().await.ok() });
  let auth_signal = move || profile.get().and_then(|_| auth_cookie.get());

  view! {
    <Suspense fallback=|| {
      view! { <p>"Loading (Suspense Fallback)..."</p> }
    }>
      <Show
        when=move || auth_signal().map(|auth_cookie| auth_cookie == "bob").unwrap_or(false)
        fallback= ||view! {
          <button
            class="transition duration-75 rounded-md hover:bg-violet8 bg-violet9 active:bg-violet10 px-1.5 py-1 text-white text-sm font-semibold w-fit"
            on:click=move |_| {
              spawn_local(async move {
                _ = login().await;
                // profile.refetch();
              });
            }
          >
            "Login"
          </button>
        }
      >
        <div class="flex gap-2 items-center">
          <span>"hello bob"</span>
          <button
            class="transition duration-75 rounded-md hover:bg-violet8 bg-violet9 active:bg-violet10 px-1.5 py-1 text-white text-sm font-semibold"
            on:click=move |_| {
              spawn_local(async move {
                _ = logout().await;
                // profile.refetch();
              });
            }
          >
            "Logout"
          </button>
        </div>
      </Show>
    </Suspense>
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
      SameSite,
    },
    http::header,
    http::header::HeaderValue,
  };
  use leptos_actix::ResponseOptions;
  use std::ops::Add;

  let mut cookie = Cookie::build("auth", "bob")
    .max_age(Duration::days(7))
    .expires(OffsetDateTime::now_utc() + Duration::days(7))
    .same_site(SameSite::Lax)
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
      SameSite,
    },
    http::header,
    http::header::HeaderValue,
  };
  use leptos_actix::ResponseOptions;
  use std::ops::Add;

  let mut cookie = Cookie::build("auth", "")
    .max_age(Duration::seconds(0))
    .expires(OffsetDateTime::now_utc() - Duration::seconds(1))
    .same_site(SameSite::Lax)
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

  view! { <h1>"Not Found"</h1> }
}
