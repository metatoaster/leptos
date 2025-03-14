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

    Ctx::provide();

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

                    // <div>"Start After CtxView"</div>
                    // <CtxView/>
                    // <div>"End After CtxView"</div>
                </SyncAwait>

                <div>"End of main"</div>
            </main>
        </Router>
    }
}

#[derive(Clone, Debug, Default)]
pub struct Ctx {
    inner: Option<Resource<Result<String, ServerFnError>>>,
    refresh: RwSignal<usize>,
}

impl Ctx {
    /// Clear the resource in the portlet.  The component using this
    /// may decide to not render anything.
    pub fn clear(&mut self) {
        self.refresh.try_update(|n| *n += 1);
        self.inner = None;
    }

    /// Set the resource for this portlet.
    pub fn set(&mut self, value: Resource<Result<String, ServerFnError>>) {
        self.refresh.try_update(|n| *n += 1);
        self.inner = Some(value);
    }

    /// The reason why there is no constructor provided and only done so
    /// via signal is to have these contexts function as a singleton.
    pub fn provide() {
        let (rs, ws) = signal(Ctx {
            inner: None,
            refresh: RwSignal::new(0),
        });
        provide_context(rs);
        provide_context(ws);
    }
}

#[component]
fn CtxView() -> impl IntoView {
    let rs = expect_context::<ReadSignal<Ctx>>();
    #[cfg(feature = "ssr")]
    let waiter = Waiter::maybe();
    let resource = Resource::new_blocking(
        {
            let refresh = rs.get_untracked().refresh.clone();
            move || {
                leptos::logging::log!("into_render suspend resource signaled!");
                refresh.get()
            }
        },
        move |id| {
            #[cfg(feature = "ssr")]
            let waiter = waiter.clone();
            leptos::logging::log!("refresh id {id}");
            let rs = rs.clone();
            async move {
                #[cfg(feature = "ssr")]
                waiter.subscribe().wait().await;
                let ctx = rs.get();
                leptos::logging::log!("ctx = {ctx:?}");
                if let Some(resource) = ctx.inner {
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

    on_cleanup(move || {
        // a bare set_ctx will result in the cleanup triggering the
        // re-render immediately which results in the resource that
        // might be set later in another component from triggering the
        // actual render.
        leptos::logging::log!("Running on_cleanup in Foo");
        Effect::new(move || {
            leptos::logging::log!("set_ctx with None in Effect of Foo on_cleanup");
            set_ctx.update(|c| c.clear());
        });
    });

    set_ctx.update(move |c| {
        leptos::logging::log!("set_ctx with Some(Resource) in Foo hook");
        c.set(Resource::new_blocking(
            move || (),
            move |_| async move {
                // emulate access to other resources/server_fn
                // serverfn().await?;
                Ok("set_ctx in Foo".to_string())
            },
        ))
    });
    view! {
        <h1>"Foo"</h1>
    }
}

#[component]
fn Bar() -> impl IntoView {
    let set_ctx = expect_context::<WriteSignal<Ctx>>();

    on_cleanup(move || {
        leptos::logging::log!("Running on_cleanup in Bar");
        Effect::new(move || {
            leptos::logging::log!("set_ctx with None in Effect of Bar on_cleanup");
            set_ctx.update(|c| c.clear());
        });
    });

    set_ctx.update(move |c| {
        leptos::logging::log!("set_ctx with Some(Resource) in Bar hook");
        c.set(Resource::new_blocking(
            move || (),
            move |_| async move {
                // emulate access to other resources/server_fn
                // serverfn().await?;
                Ok("set_ctx in Bar".to_string())
            },
        ))
    });
    view! {
        <h1>"Bar"</h1>
    }
}
