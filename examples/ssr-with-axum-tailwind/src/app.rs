use crate::error_template::{AppError, ErrorTemplate};
use crate::primitives::PrimitivesShowcase;
use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::*;
use leptos_use::use_cookie;

use codee::string::FromToStringCodec;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {


        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/ssr-with-axum-tailwind.css"/>

        // sets the document title
        <Title text="Leptix SSR (Axum)"/>

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! {
                <ErrorTemplate outside_errors/>
            }
            .into_view()
        }>
            <main class="dark:bg-[#111113] p-4 flex flex-col gap-2 text-mauve11 dark:text-white">
                <Routes>
                    <Route path="" view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    view! {
        <Auth />
        <PrimitivesShowcase />
    }
}

#[component]
fn Auth() -> impl IntoView {
    let (auth_cookie, _) = use_cookie::<String, FromToStringCodec>("auth");
    let profile = create_resource(|| (), |_| async move { get_profile().await.ok() });
    let auth_signal = move || profile.get().or(Some(auth_cookie.get())).flatten();

    view! {
    <Suspense fallback=move || view! { <p>"Loading (Suspense Fallback)..."</p> }>
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
          </Suspense>
    }
}

#[server]
async fn get_profile() -> Result<String, ServerFnError> {
    use axum::extract::Request;
    use leptos::server_fn::error::NoCustomError;
    use leptos_axum::extract;

    let foo = extract::<axum_extra::extract::cookie::CookieJar>()
        .await?
        .get("auth")
        .ok_or(ServerFnError::<NoCustomError>::ServerError(
            "unauthorized".to_string(),
        ))?
        .value()
        .to_string();

    Ok(foo)
}

#[server]
async fn login() -> Result<(), ServerFnError> {
    use axum::{http::header, http::header::HeaderValue};
    use axum_extra::extract::cookie::{Cookie, SameSite};
    use leptos_axum::ResponseOptions;
    use std::ops::Add;
    use time::{Duration, OffsetDateTime, Time};

    let mut cookie = Cookie::build(("auth", "bob"))
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
    use axum::{http::header, http::header::HeaderValue};
    use axum_extra::extract::cookie::{Cookie, SameSite};
    use leptos_axum::ResponseOptions;
    use std::ops::Add;
    use time::{Duration, OffsetDateTime, Time};

    let mut cookie = Cookie::build(("auth", ""))
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
