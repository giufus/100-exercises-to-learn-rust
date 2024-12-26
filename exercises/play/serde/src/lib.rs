
#[derive(Debug, serde::Deserialize, PartialEq, Eq)]
pub struct Game {
    pub title: String,
    pub price: usize,
    pub year: u32,
}


impl Game {
    pub fn new(title: String, price: usize, year: u32) -> Self {
        Self { title, price, year }
    }
}

#[cfg(test)]
mod tests {
    use serde_json::{json, Error, Value};
    use super::*;
    

    #[test]
    fn test_from_json_to_struct() {
        let game_obj = Game::new("Doom".into(), 5000, 2004);
        let game_str = r#"
        {"title":"Doom","price":5000,"year":2004}
        "#;

        let deser_obj = serde_json::from_str(game_str).unwrap();
        assert_eq!(game_obj, deser_obj);
    }

    #[test]
    fn test_from_json_to_serde_value() {
        // Creating a Value using json! macro
        let data: Value = json!({
            "title": "Doom",
            "year": 2004,
            "price": 5000,
            "tags": ["action", "shooter"],
            "available": true,
        });

        // Accessing fields in Value
        if let Some(name) = data.get("name") {
            println!("Game Name: {}", name);
        }

        // Iterating over an array
        if let Some(tags) = data.get("tags").and_then(Value::as_array) {
            for tag in tags {
                println!("Tag: {}", tag);
            }
        }

        // Serializing back to JSON string
        let json_string = serde_json::to_string(&data).unwrap();
        println!("JSON String: {}", json_string);

        let result = serde_json::from_value::<Game>(data);
        dbg!(result);
    }
    
}
