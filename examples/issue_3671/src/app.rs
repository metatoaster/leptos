use leptos::prelude::*;
use leptos_meta::{MetaTags, *};
use leptos_router::{
    components::{Route, Router, Routes},
    path, SsrMode,
};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <AutoReload options=options.clone()/>
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    let fallback = || view! { "Page not found." }.into_view();
    view! {
        <Title text="Issue 3671 demo"/>
        <Router>
            <main>
                <Routes fallback>
                    <Route path=path!("") view=HomePage ssr=SsrMode::Async/>
                </Routes>
            </main>
        </Router>
    }
}

#[server]
async fn server_call() -> Result<(), ServerFnError> {
    tokio::time::sleep(std::time::Duration::from_millis(50)).await;
    Ok(())
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
struct Ctx(Option<()>);

#[component]
fn HomePage() -> impl IntoView {
    let (_, set_ctx) = signal(Ctx(None));

    // #[cfg(not(feature = "ssr"))]
    on_cleanup(move || {
        leptos::logging::log!("on_cleanup");
        set_ctx.set(Ctx(None));
    });

    let hook = move || {
        let resource = Resource::new_blocking(
            move || (),
            move |_| async move { server_call().await },
        );
        Suspend::new(async move {
            let _ = resource.await.map(|_| {
                set_ctx.set(Ctx(Some(())))
            });
        })
    };
    view! {
        <h1>"Home Page"</h1>
        <Suspense>{hook}</Suspense>
    }
}
