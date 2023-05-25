use leptos::*;
use leptos_router::*;

#[component]
pub fn MetricsSharing(cx: Scope) -> impl IntoView {
    let test_action = create_server_action::<ReadPosts>(cx);
    view! { cx,
        <h2> "Web Sharing" </h2>
        <ActionForm action=test_action>
            <input type="input" name="how_many" value={2}/>
            <input type="submit" value="X"/>
        </ActionForm>

    }
}

#[server(ReadPosts, "/api")]
pub async fn read_posts(
    cx: Scope,
    how_many: u8,
) -> Result<Vec<usize>, ServerFnError> {
    use sqlx::{Pool, Sqlite};
    let Some(pool) = use_context::<Pool<Sqlite>>(cx) else {
        return Err(ServerFnError::ServerError("Sqlite pool unavailable".to_string()));
    };
    log!("THis Many: {}.\nPool {:?}", how_many, pool);
    Ok(vec![])
}
