use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::*;
use leptos_use::use_cookie;

use codee::string::FromToStringCodec;

use crate::primitives::PrimitivesShowcase;

#[component]
pub fn App() -> impl IntoView {
  provide_meta_context();

  view! {
      <Stylesheet id="leptos" href="/pkg/ssr-with-islands-actix-tailwind.css"/>
      <Title text="Leptix SSR (Islands)"/>

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
fn HomePage() -> impl IntoView {
  view! {
      <Auth/>
      <PrimitivesShowcase/>
  }
}

#[island]
fn Auth() -> impl IntoView {
  let (auth_cookie, _) = use_cookie::<String, FromToStringCodec>("auth");
  let profile = create_resource(|| (), |_| async move { get_profile().await.ok() });
  let auth_signal = move || profile.get().or(Some(auth_cookie.get())).flatten();

  view! {
      <Suspense fallback=move || {
          view! { <p>"Loading (Suspense Fallback)..."</p> }
      }>
          {move || {
              if auth_signal().map(|auth_cookie| auth_cookie == "bob").unwrap_or(false) {
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
              }
          }}

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
