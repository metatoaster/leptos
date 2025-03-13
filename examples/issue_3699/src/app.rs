use leptos::prelude::*;
use leptos_meta::{MetaTags, *};
use leptos_router::{
    components::{A, Route, Router, Routes},
    path, SsrMode,
};

use crate::sync_await::SyncAwait;
#[cfg(feature = "ssr")]
use crate::sync_await::ssr::Waiter;

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

    let (rs, ws) = signal(Ctx(None));
    provide_context(rs);
    provide_context(ws);

    let fallback = || view! { "Page not found." }.into_view();
    view! {
        <Title text="Demo"/>
        <Router>
            <main>
                <nav>
                    <ul>
                        <li><A href="/">"Root"</A></li>
                        <li><A href="/foo">"Foo"</A></li>
                        <li><A href="/bar">"Bar"</A></li>
                    </ul>
                </nav>

                <SyncAwait>
                    <div>"Start Before CtxView"</div>
                    <CtxView/>
                    <div>"End Before CtxView"</div>

                    <div>"Routes"</div>
                    <Routes fallback>
                        <Route path=path!("") view=HomePage ssr=SsrMode::Async/>
                        <Route path=path!("/foo") view=Foo ssr=SsrMode::Async/>
                        <Route path=path!("/bar") view=Bar ssr=SsrMode::Async/>
                    </Routes>

                    <div>"Start After CtxView"</div>
                    <CtxView/>
                    <div>"End After CtxView"</div>
                </SyncAwait>

                <div>"End of main"</div>
            </main>
        </Router>
    }
}

#[derive(Clone, Debug)]
struct Ctx(Option<Resource<Result<String, ServerFnError>>>);

// `PartialEq` is required for `PortletCtx<T>` in order for it to be
// enclosed inside a `ReadSignal`.  Since implementing `PartialEq` for
// `ArcResource<...> is not exactly feasible, and that what this use
// case ultimately cares about is whether or not there is some resource
// being assigned, thus comparison using `.is_none()` is sufficient, and
// assume all resources are not equal to another.
impl PartialEq for Ctx {
    fn eq(&self, other: &Self) -> bool {
        if self.0.is_none() {
            other.0.is_none()
        } else {
            false
        }
    }
}

#[component]
fn CtxView() -> impl IntoView {
    let rs = expect_context::<ReadSignal<Ctx>>();
    view! {
        <Transition>{
            move || {
                #[cfg(feature = "ssr")]
                let waiter = Waiter::maybe();
                Suspend::new(async move {
                    let result = Resource::new_blocking(
                        {
                            let rs = rs.clone();
                            move || {
                                leptos::logging::log!("into_render suspend resource signaled!");
                                rs.get()
                            }
                        },
                        move |ctx| {
                            #[cfg(feature = "ssr")]
                            let waiter = waiter.clone();
                            async move {
                                #[cfg(feature = "ssr")]
                                waiter.subscribe().wait().await;
                                leptos::logging::log!("ctx = {ctx:?}");
                                if let Some(resource) = ctx.0 {
                                    Some(resource.await)
                                } else {
                                    None
                                }
                            }
                        },
                    ).await;
                    if let Some(result) = result {
                        let value = result?;
                        leptos::logging::log!("returning actual view");
                        Ok::<_, ServerFnError>(
                            Some(view! {
                                <div>"The value is: "{value}</div>
                            }
                            .into_any())
                        )
                    } else {
                        Ok(None)
                    }
                })
            }
        }</Transition>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    view! {
        <h1>"Home Page"</h1>
    }
}

#[component]
fn Foo() -> impl IntoView {
    let set_ctx = expect_context::<WriteSignal<Ctx>>();

    on_cleanup(move || {
        leptos::logging::log!("set_ctx with None in Effect of Foo on_cleanup");
        set_ctx.set(Ctx(None));
    });

    let hook = move || {
        leptos::logging::log!("set_ctx with Some(Resource) in Foo hook");
        set_ctx.set(Ctx(Some(Resource::new_blocking(
            move || (),
            move |_| async move {
                // hypothetical access to other resources/server_fn call here
                Ok("set_ctx in Foo".to_string())
            },
        ))))
    };
    view! {
        <h1>"Foo"</h1>
        {hook}
    }
}

#[component]
fn Bar() -> impl IntoView {
    let set_ctx = expect_context::<WriteSignal<Ctx>>();

    on_cleanup(move || {
        leptos::logging::log!("set_ctx with None in Effect of Bar on_cleanup");
        set_ctx.set(Ctx(None));
    });

    let hook = move || {
        leptos::logging::log!("set_ctx with Some(Resource) in Bar hook");
        set_ctx.set(Ctx(Some(Resource::new_blocking(
            move || (),
            move |_| async move {
                // hypothetical access to other resources/server_fn call here
                Ok("set_ctx in Bar".to_string())
            },
        ))))
    };
    view! {
        <h1>"Bar"</h1>
        {hook}
    }
}
