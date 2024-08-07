use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{
    components::{Redirect, Route, Router, Routes},
    StaticSegment,
};


pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[server]
pub async fn goto_target() -> Result<String, ServerFnError> {
    tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    leptos_axum::redirect("/target");
    Ok("/target".to_string())
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    view! {
        <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
        <Stylesheet id="leptos" href="/pkg/axum_redirect_demo.css"/>
        <Router>
            <header>
                <h1>"Redirect Example"</h1>
            </header>
            <main>
                <Routes fallback=|| "error".into_view()>
                    <Route path=StaticSegment("") view=Example/>
                    <Route path=StaticSegment("demo") view=DemoPage />
                    <Route path=StaticSegment("target") view=|| "at the target".into_view() />
                </Routes>
            </main>
        </Router>
    }
}

#[component]
pub fn Example() -> impl IntoView {
    view! {
        <a href="/demo">"run demo"</a>
    }
}

#[component]
pub fn DemoPage() -> impl IntoView {
    let target = Resource::new(|| (), |_| async move { goto_target().await });

    view! {
        <Suspense fallback=move || view! { <p>"Loading..."</p> }>
            <ErrorBoundary fallback=|_| view! { <p>"Error"</p> }>
                // <span>"Redirecting to "{target}</span>
                <span>"Redirecting to "{
                    move || target.get().map(|target| { target.map(|target| {
                        view! {
                            {target.clone()}
                            <Redirect
                                path=target.clone()
                                options=leptos_router::NavigateOptions { replace: true, ..Default::default() }
                                />
                        }
                    })})
                }</span>
            </ErrorBoundary>
        </Suspense>
    }
}
