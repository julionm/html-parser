use std::{collections::HashMap};

fn process_attr(line: &str) -> Result<HashMap<&str, &str>, &str> {
    let mut new_line = line;
    let mut result: HashMap<&str, &str> = HashMap::new();
    
    while new_line.len() > 0 {
        match new_line.find('=') {
            Some(idx) => {
                let identifier = &new_line[..idx]; // id
                new_line = &new_line[idx+2..]; // removing =" from the start of the string

                let last_value_index = match new_line.find('"') {
                    Some(idx2) => idx2,
                    None => return Err("Malformed attributes string")
                };

                let value = &new_line[..last_value_index];

                new_line = if last_value_index + 2 > new_line.len() { "" } 
                            else { &new_line[last_value_index+2..] };

                result.insert(identifier, value);

            },
            None => {
                new_line = "";
            }
        }

    }

    Ok(result)
}

fn main() {



}

#[cfg(test)]
mod tests {
    use crate::process_attr;

    use std::collections::HashMap;

    #[test]
    fn test_attribute_generation() {
        let attrs = "id=\"some_id\" style=\"background-color: red;\" class=\"flex-1 direction-column\" data-flex=\"aushauhsuahsua\"";

        let mut result = HashMap::new();

        result.insert("id", "some_id");
        result.insert("style", "background-color: red;");
        result.insert("class", "flex-1 direction-column");
        result.insert("data-flex", "aushauhsuahsua");

        assert_eq!(process_attr(attrs), Ok(result));
    }

}

