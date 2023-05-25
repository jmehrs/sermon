use app::*;
use axum::{extract::{Extension, Path, RawQuery}, routing::{post, get}, Router, response::{IntoResponse, Response}, http::{HeaderMap, Request}, body::Body as AxumBody};
use fileserv::file_and_error_handler;
use leptos::*;
use leptos_axum::{generate_route_list, LeptosRoutes, handle_server_fns_with_context};
use std::sync::Arc;
use sqlx::{Pool, sqlite::{SqlitePoolOptions, SqliteConnectOptions, Sqlite}};
use std::str::FromStr;
use thiserror::Error;

pub mod fileserv;

async fn server_fn_handler(
    Extension(pool): Extension<Pool<Sqlite>>,
    path: Path<String>,
    headers: HeaderMap,
    raw_query: RawQuery,
    request: Request<AxumBody>
) -> impl IntoResponse {
    handle_server_fns_with_context(
        path,
        headers,
        raw_query,
        move |cx| provide_context(cx, pool.clone()),
        request
    ).await
}

async fn leptos_route_handler(
    Extension(options): Extension<Arc<LeptosOptions>>,
    Extension(pool): Extension<Pool<Sqlite>>,
    req: Request<AxumBody>
) -> Response {
    let handler = leptos_axum::render_app_async_with_context(
        (*options).clone(),
        move |cx| provide_context(cx, pool.clone()),
        |cx| view! { cx, <App/> }
    );
    handler(req).await.into_response()
}

#[derive(Error, Debug)]
pub enum ApplicationError {
    #[error("Error connecting to the DB")]
    DBConectionError(#[from] sqlx::Error),
    #[error("Error migrating the DB")]
    DBMigrationError(#[from] sqlx::migrate::MigrateError),
}

#[tokio::main]
async fn main() -> Result<(), ApplicationError> {
    // Start up a basic logger
    simple_logger::init_with_level(log::Level::Debug).expect("couldn't initialize logging");

    // Establish connection to the DB
    let pool = {
        let options = SqliteConnectOptions::from_str("sqlite:data.db")?
            .create_if_missing(true);
        SqlitePoolOptions::new()
            .max_connections(5)
            .connect_with(options)
            .await?
    };
    sqlx::migrate!("../migrations").run(&pool).await?;

    // Register app server functions
    register_server_functions();

    let conf = get_configuration(None).await.unwrap();
    let leptos_options = Arc::new(conf.leptos_options);
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(|cx| view! { cx, <App/> }).await;

    // build our application with a route
    let app = Router::new()
        .route("/api/*fn_name", post(server_fn_handler))
        .leptos_routes_with_handler(routes, get(leptos_route_handler))
        .fallback(file_and_error_handler)
        .layer(Extension(pool))
        .layer(Extension(leptos_options));

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    log!("listening on http://{}", &addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
    Ok(())
}
