use std::collections::HashMap;

pub fn public_function() {
    println!("called projectDJ's `public_function()`");
}

pub fn load<T>(_s: &str) -> HashMap<&str, T> {
    return HashMap::new();
}

pub fn dump<T: std::fmt::Debug>(dict: HashMap<&str, T>) -> String {
    // (1) Map of key-value pairs of string, T
    // "{"key1":"value1","key2":"value2"}"
    let mut result = "{".to_owned();
    let mut idx: i32 = 0;

    for (key, value) in dict {
        if idx != 0 {
            result.push_str(",")
        }
        let key_value = format!("\"{}\":{:?}", key, value);
        result.push_str(&key_value);

        idx += 1;
    }

    result.push_str("}");

    return result;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
