use std::collections::HashMap;

struct Crud {
    data: HashMap<u32, String>,
}

impl Crud {

    fn new() -> Self {
        Crud {
            data: HashMap::new(),
        }
    }

    fn create(&mut self, id: u32, value: String) {
        self.data.insert(id, value);
        println!("Created: {} -> {}", id, value);
    }

    fn read(&self, id: u32) {
        match self.data.get(&id) {
            Some(v) => println!("Read: {}", v),
            None => println!("Record not found"),
        }
    }

    fn update(&mut self, id: u32, value: String) {
        self.data.insert(id, value);
        println!("Updated: {} -> {}", id, value);
    }

    fn delete(&mut self, id: u32) {
        self.data.remove(&id);
        println!("Deleted: {}", id);
    }
}

fn main() {

    let mut crud = Crud::new();

    crud.create(1, "First record".to_string());
    crud.read(1);

    crud.update(1, "Updated record".to_string());
    crud.read(1);

    crud.delete(1);
}
