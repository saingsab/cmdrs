use std::collections::HashMap;
fn main() {
    let mut arguments = std::env::args().skip(1);
    let key = arguments.next().unwrap();
    let value = arguments.next().unwrap();
    println!("The key is {}", key);
    println!("The key is {}", value);

    // write file
    // let contents = format!("{}\t{}\n", key, value);
    // let write_result = std::fs::write("cmdrs.db", contents);
    
    // Using Match partern to handling ERROR
    // match write_result {
    //     Ok(()) => {

    //     }
    //     Err(e) => {
    //         println!("Error : {}", e)
    //     }
    // }

    // Working with struct and method
    let mut database = Database::new().expect("Creating db failed");
    database.insert(key.to_uppercase(), value.clone());
    database.insert(key, value); 
    database.flush().unwrap();
}

struct Database {
    map: HashMap<String, String>,
}

impl Database {
    fn new() -> Result<Database, std::io::Error> {
        // read the cmdrs file
        // let contens = match std::fs::read_to_string("cmdrs.db") {
        //     Ok(c) => c,
        //     Err(e) => {
        //         return Err(e);
        //     }
        // };
        // Write an equal to above function
        let contens = std::fs::read_to_string("cmdrs.db")?;
        let mut map = HashMap::new();

        for line in contens.lines() {
            let mut chunks = line.splitn(2, "\t"); //Split into two from the tap
            let key =  chunks.next().expect("No Key!");
            let value =  chunks.next().expect("No value!");
            map.insert(key.to_owned(), value.to_owned());
        }
        // populate our map
        Ok(Database { map: map})
    }

    fn insert(&mut self, key: String, values: String){
        self.map.insert(key, values);
    }

    fn flush(self) -> std::io::Result<()> {
        let mut contents = String::new();
        for pairs in self.map {
            let cmdpair = format!("{}\t{}\n", pairs.0, pairs.1);
            contents.push_str(&cmdpair);
        }
        std::fs::write("cmdrs.db", contents)
    }
}
