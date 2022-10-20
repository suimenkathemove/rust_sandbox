pub type Id = u64;

pub type Text = String;

pub struct Todo {
    id: Id,
    text: Text,
}

impl Todo {
    pub fn new(id: Id, text: Text) -> Self {
        Self { id, text }
    }
}

pub struct CreateTodo {
    text: Text,
}

pub struct UpdateTodo {
    text: Text,
}

pub trait TodoRepositoryTrait {
    fn index(&self) -> Vec<Todo>;

    fn create(&self, payload: CreateTodo) -> Todo;

    fn get(&self, id: Id) -> Todo;

    fn update(&self, id: Id, payload: UpdateTodo) -> Todo;

    fn delete(&self, id: Id);
}
