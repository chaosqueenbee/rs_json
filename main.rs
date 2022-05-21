use std::collections::HashMap;

fn main() {
    let map: HashMap<&str, i32> = HashMap::new();
    println!("{}", rs_json::dump::<i32>(map));
}