use axum::response::Redirect;

pub async fn docs() -> Redirect {
    Redirect::permanent("https://documenter.getpostman.com/view/33457905/2sAYBYgVpP")
}
