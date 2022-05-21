use std::collections::HashMap;

fn main() {
    let mut map: HashMap<&str, i32> = HashMap::new();
    map.insert("Dalton", 1);
    map.insert("Jasmine", 0);
    println!("{}", rs_json::dump::<i32>(map));
}