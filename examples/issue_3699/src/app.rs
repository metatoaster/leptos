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
        // leptos::logging::log!("PartialEq::eq for Ctx");
        // if self.0.is_none() {
        //     other.0.is_none()
        // } else {
        //     true
        // }
        false
    }
}

#[component]
fn CtxView() -> impl IntoView {
    let rs = expect_context::<ReadSignal<Ctx>>();
    #[cfg(feature = "ssr")]
    let waiter = Waiter::maybe();
    let resource = Resource::new_blocking(
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
            // let rs = rs.clone();
            async move {
                // let ctx = rs.get();
                #[cfg(feature = "ssr")]
                waiter.subscribe().wait().await;
                leptos::logging::log!("ctx = {ctx:?}");
                if let Some(resource) = ctx.0 {
                    leptos::logging::log!("resource returning Some");
                    Some(resource.await)
                } else {
                    leptos::logging::log!("resource returning None");
                    None
                }
            }
        },
    );
    let suspend = move || { Suspend::new(async move {
        let result = resource.await;
        if let Some(result) = result {
            let value = result?;
            leptos::logging::log!("Suspend view returning Some");
            Ok::<_, ServerFnError>(
                Some(view! {
                    <div>"The value is: "{value}</div>
                }
                .into_any())
            )
        } else {
            leptos::logging::log!("Suspend view returning None");
            Ok(None)
        }
    })};

    view! {
        <Transition>{
            move || suspend()
        }</Transition>
    }
}

#[server]
async fn serverfn() -> Result<(), ServerFnError> {
    tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    Ok(())
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

    // on_cleanup(move || {
    //     leptos::logging::log!("set_ctx with None in Effect of Foo on_cleanup");
    //     set_ctx.set(Ctx(None));
    // });
    on_cleanup(move || {
        // a bare set_ctx will result in the cleanup triggering the
        // re-render immediately which results in the resource that
        // might be set later in another component from triggering the
        // actual render.
        // set_ctx.set(Ctx(None));
        leptos::logging::log!("Running on_cleanup in Foo");
        Effect::new(move || {
            leptos::logging::log!("set_ctx with None in Effect of Foo on_cleanup");
            set_ctx.set(Ctx(None));
        });
    });

    let hook = move || {
        leptos::logging::log!("set_ctx with Some(Resource) in Foo hook");
        set_ctx.set(Ctx(Some(Resource::new_blocking(
            move || (),
            move |_| async move {
                // hypothetical access to other resources/server_fn call here
                serverfn().await?;
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
        leptos::logging::log!("Running on_cleanup in Bar");
        Effect::new(move || {
            leptos::logging::log!("set_ctx with None in Effect of Bar on_cleanup");
            set_ctx.set(Ctx(None));
        });
    });

    let hook = move || {
        leptos::logging::log!("set_ctx with Some(Resource) in Bar hook");
        set_ctx.set(Ctx(Some(Resource::new_blocking(
            move || (),
            move |_| async move {
                // hypothetical access to other resources/server_fn call here
                serverfn().await?;
                Ok("set_ctx in Bar".to_string())
            },
        ))))
    };
    view! {
        <h1>"Bar"</h1>
        {hook}
    }
}
