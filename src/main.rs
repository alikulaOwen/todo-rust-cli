use std::collections::HashMap;

fn main() {
    let action = std::env::args().nth(1).expect("Please specify an action");
    let item = std::env::args().nth(2).expect("Please specify an item ");
    println!("{:?} {:?}", action, item);

    let mut todo = Todo::new().expect("Failed to inialize Todo CLI");

    // match action.as_ref() {
    //     "add" => todo.add_todo(item),
    //     "complete" => todo.complete(item),
    //     "list" => todo.list_todos(),
    //     _ => println!("Unknown action"),
    // }


    if action == "add" {
        todo.add_todo(item);
        match todo.save() {
            Ok(_) => println!("Saved"),
            Err(e) => println!("Error Occured while saving: {:?}", e)
        }
    } else if action == "complete" {
        todo.complete(item);
    }else if action == "list" {
        todo.list_todos();
    } 
    else {
        println!("Unknown action");
    }
}

struct Todo{
    //we'll use rust built in hash map to store Key value pairs

    map: HashMap<String, bool>,

}


impl Todo{
   fn new()-> Result<Todo, std::io::Error>{
    //opendb.json
    let f = std::fs::OpenOptions::new()
    .read(true)
    .write(true)
    .create(true)
    .open("opendb.json")?;

    // seria;lize json as a hashmap
    match serde_json::from_reader(f) {
        Ok(map) => Ok(Todo{map}),
        Err(e) => {
            if e.is_eof() {
                Ok(Todo{map: HashMap::new()})
            } else {
                Err(e.into())
            }
        },

    }

   }

    fn add_todo(&mut self, key: String){
        self.map.insert(key, true);
    }
   
    //tabnine
    // fn in_progress(&self) -> impl Iterator<Item = (&String, &bool)>{
    //     self.map.iter().filter(|(_, v)| **v)
    // }
    fn list_todos(&self) {
        if self.map.len() == 0 {
            println!("No todos");
        } else {
            for (key, value) in self.map.iter() {
                if *value {
                    println!("{}: In progress", key);
                } else {
                    println!("{}: Completed", key);
                }
            }
        }
      
    }

    fn complete(&mut self, key: String){
        self.map.insert(key, true);
    }
    fn save(&self) -> Result<(), std::io::Error>{
        let f = std::fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .open("opendb.json")?;

        serde_json::to_writer_pretty(f, &self.map)?;
        Ok(())
    }
   
}