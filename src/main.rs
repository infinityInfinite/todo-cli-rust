use std::collections::HashMap;
use std::str::FromStr;
use std::io::Read;
struct Todo {
    // use rust built in hashmap to store key - val pairs
    map : HashMap<String,bool>
}

impl Todo {
    fn insert(&mut self , key : String){
        // insert a new item to our map
        // we pass true as value

        self.map.insert(key,true);
    }
    fn save(self) -> Result<(),std::io::Error> {
        let mut content = String::new();
        for (k , v) in self.map {
            let record = format!("{}\t{}\n", k, v);
            content.push_str(&record);
        }
        std::fs::write("db.txt",content)
    }
    fn new() -> Result<Todo,std::io::Error> {
        let mut f = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .read(true)
            .open("db.txt")?;
        let mut content = String::new();
        f.read_to_string(&mut content)?;
        let map: HashMap<String,bool> = content
            .lines()
            .map(|line| line.splitn(2, '\t').collect::<Vec<&str>>())
            .map(|v| (v[0], v[1]))
            .map(|(k, v)| (String::from(k), bool::from_str(v).unwrap())).collect();

        Ok(Todo { map })
    } 
    fn complete(&mut self,key:&String) -> Option<()> {
        match self.map.get_mut(key){
            Some(v) => Some(*v = false),
            None => None
        }
    }

    fn clear() -> std::io::Result<()> {
        std::fs::write("db.txt","")?;
        Ok(())
    }
    fn delete(&mut self, key:&String) -> bool{
        match self.map.remove_entry(key){
            Some(_) => true,
            None => false
        }
    }
}

fn main() {
    let action = std::env::args().nth(1).expect("Please specify an action !");
    let item = std::env::args().nth(2).expect("Please specify an item !");

    let mut todo = Todo::new().expect("Initialization of db failed !");
    if action == "add" {
        todo.insert(item);
        match todo.save() {
            Ok(_) => println!("todo saved"),
            Err(why) => println!("An error occurred: {}", why),
        }
    }else if action == "complete" {
        match todo.complete(&item){
            None => println!("'{}' is not present in the list", item),
            Some(_) => match todo.save() {
                    Ok(_) => println!("todo saved"),
                    Err(why) => println!("An error occurred: {}", why)
            }
        
        }
    }else if action == "clear"{
        match Todo::clear() {
            Ok(_) => println!("todo saved"),
            Err(why) => println!("An error occured {why}")
        }
    }else if action == "delete"{
        match todo.delete(&item){
            true => match todo.save() {
                Ok(_) => println!("todo saved !!"),
                Err(why) => println!("An error occured : {}",why)
            },
            false => println!("item not found !!")
        }
    }
}
