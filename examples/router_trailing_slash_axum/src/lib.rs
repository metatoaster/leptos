use leptos::*;
use leptos_router::*;

#[component]
pub fn MainRoutes() -> impl IntoView {
    view! {
        <Router>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                    <ItemRoutes/>
                </Routes>
            </main>
        </Router>
    }
}

// This is almost the desired definition.
// The desired definition would use trailing_slash=Redirect, but it results in duplicate routes
#[component(transparent)]
pub fn ItemRoutes() -> impl IntoView {
    view! {
        <Route path="/item" view=ItemTop>
            <Route path="/:id/" view=ItemView trailing_slash=TrailingSlash::Exact/>
            <Route path="/" view=ItemListing trailing_slash=TrailingSlash::Exact/>
        </Route>
    }
}

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <h1>Home</h1>
        <ul>
            <li><A href="item/">"Item Listing"</A></li>
        </ul>
    }
}

#[component]
pub fn ItemTop() -> impl IntoView {
    view! {
        <h1>"Item Management"</h1>
        <Outlet/>
    }
}

#[component]
pub fn ItemListing() -> impl IntoView {
    view! {
        <h2>"Listing of items"</h2>
        <ul>
            <li><A href="1/">"Item 1"</A></li>
            <li><A href="2/">"Item 2"</A></li>
        </ul>
    }
}

#[derive(Params, PartialEq, Clone, Debug)]
pub struct ItemParams {
    id: Option<String>,
}

#[component]
pub fn ItemView() -> impl IntoView {
    let params = use_params::<ItemParams>();
    let id = move || {
        params.with(|params| {
            params.as_ref()
                .map(|params| params.id.clone())
                .unwrap_or_default()
        })
    };

    view! {
        <h2>"Viewing item "{id}</h2>
    }
}
