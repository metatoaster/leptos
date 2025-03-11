use leptos::prelude::*;
use leptos_meta::{MetaTags, *};
use leptos_router::{
    components::{A, Route, Router, Routes},
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

                // Having CtxView before routes is the source of the issue
                // needing the workaround described later
                <div>"Start CtxView"</div>
                <CtxView/>
                <div>"End CtxView"</div>

                <div>"Routes"</div>
                <Routes fallback>
                    <Route path=path!("") view=HomePage ssr=SsrMode::Async/>
                    <Route path=path!("/foo") view=Foo ssr=SsrMode::Async/>
                    <Route path=path!("/bar") view=Bar ssr=SsrMode::Async/>
                </Routes>

                // CtxView after routes has less issue
                // <div>"Start CtxView"</div>
                // <CtxView/>
                // <div>"End CtxView"</div>

                <div>"End of main"</div>
            </main>
        </Router>
    }
}

#[derive(Clone, Debug)]
struct Ctx(Option<Resource<Result<String, ServerFnError>>>);

#[component]
fn CtxView() -> impl IntoView {
    let rs = expect_context::<ReadSignal<Ctx>>();
    view! {
        <Transition>{
            move || Suspend::new(async move {
                let ctx = rs.get();
                leptos::logging::log!("ctx = {ctx:?}");
                if let Some(resource) = ctx.0 {
                    let value = resource.await?;
                    leptos::logging::log!("returning actual view");
                    Ok::<_, ServerFnError>(
                        view! {
                            <div>"The value is: "{value}</div>
                        }
                        .into_any()
                    )
                } else {
                    leptos::logging::log!("returning empty view");
                    // XXX this return value will result in hydration error
                    // if CtxView comes before routes
                    // Ok::<_, ServerFnError>(().into_any())

                    // basically any response here that doesn't include some
                    // kind of html element will result in the error.

                    // a view will help with mitigating the issue, but it
                    // must return some element, not just text.
                    Ok(
                        view! {
                            // Uncomment following to use the workaround:
                            // <noscript></noscript>
                            // uncomment the following string will also show
                            // the hydration error:
                            // "<noscript></noscript>"
                        }
                        .into_any()
                    )
                }
            })
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
                Ok("set_ctx in Bar".to_string())
            },
        ))))
    };
    view! {
        <h1>"Bar"</h1>
        {hook}
    }
}
