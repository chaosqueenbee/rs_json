use std::collections::HashMap;

pub fn public_function() {
    println!("called projectDJ's `public_function()`");
}

pub fn load<T>(_s: &str) -> HashMap<&str, T> {
    return HashMap::new();
}

pub fn dump<T>(_dict: HashMap<&str, T>) -> &str {
    return "Not Implemented";
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
