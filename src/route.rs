use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};

use crate::{
    handler::{
        create_feedback_handler, delete_feedback_handler, edit_feedback_handler, get_feedback_handler,
        feedback_list_handler,
    },
    AppState,
};

use minijinja::{path_loader, Environment, Value};
use std::{collections::HashMap};

use axum::{
    response::Html,
};

pub fn create_router(app_state: Arc<AppState>) -> Router{
    // Carrega os arquivos da pasta ./templates/
    let mut env = Environment::new();
    env.set_loader(path_loader("templates"));

    // Compartilhado entre threads com Arc
    let env = Arc::new(env);

    Router::new()
        .route("/api/feedbacks/", post(create_feedback_handler))
        .route("/api/feedbacks", get(feedback_list_handler))
        .route(
            "/api/feedbacks/{id}",
            get(get_feedback_handler)
                .patch(edit_feedback_handler)
                .delete(delete_feedback_handler),
        ).route("/", get({
            let env = env.clone();
            move || index(env)
        }))
        .with_state(app_state)
}

async fn index(env: Arc<Environment<'static>>) -> Html<String> {
    let mut ctx = HashMap::new();
    ctx.insert("title".to_string(), Value::from("Minha Página Incrível"));
    ctx.insert("user".to_string(), Value::from("João"));

    let tmpl = env.get_template("index.html").unwrap();
    let rendered = tmpl.render(Value::from(ctx)).unwrap();

    Html(rendered)
}