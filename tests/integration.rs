use std::collections::HashMap;

use postgres_from_row::FromRow;
use tokio_postgres::{types::Json, Row};

#[derive(FromRow)]
#[allow(dead_code)]
pub struct Todo {
    todo_id: i32,
    text: String,
    #[from_row(flatten)]
    user: User,
    #[from_row(from_fn = "json")]
    json: HashMap<String, bool>,
}

fn json<T>(wrapper: Json<T>) -> T {
    wrapper.0
}

#[derive(FromRow)]
#[allow(dead_code)]
pub struct User {
    user_id: i32,
}

#[allow(dead_code)]
fn from_row(row: &Row) {
    let _ = Todo::from_row(row);
    let _ = Todo::try_from_row(row).unwrap();

    let _ = User::from_row(row);
    let _ = User::try_from_row(row).unwrap();
}

#[allow(dead_code)]
fn from_rows(row: &Vec<Row>) {
    let _ = Todo::from_rows(row);
    let _ = Todo::try_from_rows(row).unwrap();

    let _ = User::from_rows(row);
    let _ = User::try_from_rows(row).unwrap();
}

#[allow(dead_code)]
fn from_row_maybe(row: Option<&Row>) {
    let _ = Todo::from_row_maybe(row);
    let _ = Todo::try_from_row_maybe(row).unwrap();

    let _ = User::from_row_maybe(row);
    let _ = User::try_from_row_maybe(row).unwrap();
}
