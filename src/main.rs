use std::collections::HashMap;
use std::str::FromStr;
use std::io::Read;

fn main() {
    let action = std::env::args().nth(1).expect("Please specify an action");
    let item = std::env::args().nth(2).expect("Please specify an item");
    
    let mut todo = Todo::new().expect("Initialisation of db failed.");
    if action == "add" {
        todo.insert(item);
        match todo.save() {
            Ok(_) => println!("Task saved."),
            Err(reason) => println!("An error occurred: {}.", reason),
        }
    }
    else if action == "complete" {
        match todo.complete(&item) {
            None => println!("'{}' is not in the list", item),
            Some(_) => match todo.save() {
                Ok(_) => println!("Task updated."),
                Err(reason) => println!("An error occurred: {}", reason),
            },
        }
    }
}

struct Todo {
    map: HashMap<String, bool>,
}

impl Todo {
    fn new() -> Result<Todo, std::io::Error> {
        let mut file = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .read(true)
        .open("db.txt")?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        let map: HashMap<String, bool> = content
            .lines()
            .map(|line| line.splitn(2, '\t').collect::<Vec<&str>>())
            .map(|val| (val[0], val[1]))
            .map(|(key, val)| (String::from(key), bool::from_str(val).unwrap()))
            .collect();
        Ok(Todo { map })
    }

    fn insert(&mut self, key: String) {
        self.map.insert(key, true);
    }

    fn save(self) -> Result<(), std::io::Error> {
        let mut content = String::new();
        for (key, val) in self.map {
            let record = format!("{}\t{}\n", key, val);
            content.push_str(&record)
        }
        std::fs::write("db.txt", content)
    }

    fn complete(&mut self, key: &String) -> Option<()> {
        match self.map.get_mut(key) {
            Some(val) => Some(*val = false),
            None => None,
        }
    }
}
