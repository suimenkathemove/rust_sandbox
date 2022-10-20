//! # Arc<RwLock<>>
//!
//! スレッドセーフに書き換えをしたい場合のパターン
//!
//! 不変参照のときは複数スレッドで共有でき、可変参照のときはRwLockがスレッドを1つに制限するため安全に書き込みができる

use crate::todo::{CreateTodo, Id, Todo, TodoRepositoryTrait, UpdateTodo};
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

pub struct TodoRepository(Arc<RwLock<HashMap<Id, Todo>>>);

impl TodoRepository {
    pub fn new() -> Self {
        Self(Arc::default())
    }
}

impl TodoRepositoryTrait for TodoRepository {
    fn index(&self) -> Vec<Todo> {
        todo!();
    }

    fn create(&self, payload: CreateTodo) -> Todo {
        todo!();
    }

    fn get(&self, id: Id) -> Todo {
        todo!();
    }

    fn update(&self, id: Id, payload: UpdateTodo) -> Todo {
        todo!();
    }

    fn delete(&self, id: Id) {
        todo!();
    }
}
