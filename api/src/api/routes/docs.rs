use axum::response::Redirect;

pub async fn docs() -> Redirect {
    Redirect::permanent("https://documenter.getpostman.com/view/34393089/2sAYk7QiQZ")
}
