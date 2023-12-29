#![allow(non_snake_case, unused)]
use dioxus::prelude::*;
use dioxus_fullstack::prelude::*;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::future::IntoFuture;
use surrealdb::{
    engine::remote::ws::{Client, Ws},
    sql::Thing,
    Surreal,
};

#[cfg(feature = "ssr")]
static DB: Lazy<Surreal<Client>> = Lazy::new(Surreal::init);

fn main() -> anyhow::Result<()> {
    let launcher = LaunchBuilder::new(app);

    #[cfg(feature = "ssr")]
    {
        let launcher = launcher.incremental(
            IncrementalRendererConfig::default()
                .invalidate_after(std::time::Duration::from_secs(120)),
        );
        tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(async move {
                DB.connect::<Ws>("localhost:8000").await?;
                DB.use_ns("test").use_db("test").await?;

                launcher.launch_server().await;
                Ok::<(), anyhow::Error>(())
            });
    }
    #[cfg(not(feature = "ssr"))]
    {
        #[cfg(feature = "web")]
        launcher.launch_web();
        #[cfg(feature = "desktop")]
        launcher.launch_desktop();
    }

    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Todo {
    content: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct TodoRecord {
    id: Thing,
    content: String,
}

#[server]
async fn get_todos() -> Result<Vec<Todo>, ServerFnError> {
    println!("get_todos");
    let todos: Vec<TodoRecord> = DB.select::<Vec<TodoRecord>>("todo").await?;
    println!("got todos");
    let todos = todos
        .into_iter()
        .map(|todo| Todo {
            content: todo.content,
        })
        .collect::<Vec<_>>();
    Ok(todos)
}

#[component]
fn TodoListItem(cx: Scope, todo: Todo) -> Element {
    cx.render(rsx! { todo.content.clone() })
}

#[component]
fn TodoList(cx: Scope, todos: Vec<Todo>) -> Element {
    cx.render(rsx! {
        ul {
            todos.iter().map(|todo| rsx! {
                li { TodoListItem { todo: todo.clone() } }
            })
        }
    })
}

fn app(cx: Scope) -> Element {
    let todos = use_server_future(cx, (), |_| async { get_todos().await })?;

    let todos_val = todos.value();
    let todo_list = match todos_val.as_ref() {
        Ok(todos) => rsx! { TodoList { todos: todos.clone() } },
        Err(err) => rsx! { "oh no: {err.to_string()}" },
    };

    cx.render(rsx! {
        main {
            class: "container",
            h1 { "Todo App" }
            div {
               todo_list
            }
        }
    })
}
