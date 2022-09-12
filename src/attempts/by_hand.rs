use std::collections::HashMap;

enum ElementType {
    Tag,
    Content
}

struct Element {
    element_type: ElementType,
    tag_name: Option<String>,
    attributes: HashMap<String, String>,
    children: Vec<Box<Element>>
}

impl Element {
    fn new() -> Element {
        Element {
            element_type: ElementType::Tag,
            tag_name: None,
            attributes: HashMap::new(),
            children: Vec::new()
        }
    }
}

fn process_attr(line: &str) -> Result<HashMap<String, String>, &str> {

    // ? id="asuhaushas" style="auhsuahsuahsuahs" data-aushausas="aushausas"

    let attrs: Vec<&str> = line.split(" ").collect();
    
    let mut result: HashMap<String, String> = HashMap::new();

    for attr in attrs {

        let a: Vec<&str> = attr.split("=").collect();

        result.insert(
            String::from(a[0]), 
            String::from(a[1].trim_matches('"')
        ));

    }


    Ok(result)
}

fn handle_dom<'a>(html: &'a String) -> Result<&'a str, &'a str> {
   
    let head_index = match html.find("</head>") {
        Some(idx) => idx + 7,
        None => 0
    };

    let mut lines = html[head_index..].lines();

    let mut dom: Vec<Element> = Vec::new();

    let mut parent_node: Option<Element> = None;

    let mut parent_stack: Vec<Element> = Vec::new();

    for line in lines {

        let line = line.trim();

        // ? what this line could be?
        // ? - piece of text
        // ? - tag

        if line.starts_with("</") {
            // ! tag closing
        } else if line.starts_with("<") {
            // * tag openning

            let end_of_identifier: usize = match line.find(" ") {
                Some(idx) => idx - 1,
                None => match line.find(">") {
                    Some(idx) => idx - 1,
                    None => 0
                }
            };

            if end_of_identifier == 0 {
                return Err("HTML Malformed")
            }

            let identifier = &line[0..end_of_identifier];

            let mut new_element = Element::new();

            new_element.tag_name = Some(String::from(identifier));

            let attrs: HashMap<String, String> = match process_attr(&line[end_of_identifier..]) {
                Ok(value) => value,
                Err(err) => return Err(err)
            };

        } else if line.starts_with("<") && line.ends_with("/>") {
            // ? self-closed tag

            continue;

        } else {
            // * content-element
        }

    }


    Ok("success")
}

