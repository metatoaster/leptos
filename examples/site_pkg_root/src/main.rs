#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::Router;
    use leptos::prelude::get_configuration;
    use leptos_axum::{LeptosRoutes, RouterConfiguration};
    use site_pkg_root::app::{App, shell};

    let conf = get_configuration(None).unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let app = Router::new()
        .leptos_route_configure(
            RouterConfiguration::new()
                .app(App)
                .shell(shell)
                .enable_fs_leptos_site_root("/")
                .state(leptos_options),
        );

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    println!("{}", listener.local_addr().unwrap().port());
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

#[cfg(not(feature = "ssr"))]
pub fn main() {}
