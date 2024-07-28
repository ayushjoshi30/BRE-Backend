//A util to convert HashMaps to Json Object and vice versa
use serde_json::Value;
use std::collections::HashMap;

pub fn hashmap_to_json(map: HashMap<String, String>) -> Value {
    let mut json_obj = serde_json::Map::new();
    for (key, value) in map {
        json_obj.insert(key, Value::String(value));
    }
    Value::Object(json_obj)
}

pub fn json_to_hashmap(json: Value) -> HashMap<String, String> {
    let mut map = HashMap::new();
    for (key, value) in json.as_object().unwrap() {
        map.insert(key.to_string(), value.as_str().unwrap().to_string());
    }
    map
}