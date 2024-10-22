use rustc_hash::FxHashMap;
use serde_json::{json, Value};

/// FxHashMapの内容をフラットに出力
pub fn _display_flat_map(map: &FxHashMap<String, String>) {
    for (key, value) in map {
        println!("{} {}", key, value);
    }
}

/// FxHashMapの内容をJSON形式出力（ネスト対応、整形出力）
pub fn display_json_map(map: &FxHashMap<String, String>) {
    let mut json_map: serde_json::Map<String, Value> = serde_json::Map::new();

    // 再帰的にネストしたマップを構築する関数
    fn insert_nested(map: &mut serde_json::Map<String, Value>, key_parts: &[&str], value: &str) {
        if key_parts.len() == 1 {
            map.insert(key_parts[0].to_string(), json!(value));
        } else {
            let entry: &mut Value = map
                .entry(key_parts[0].to_string())
                .or_insert_with(|| json!(serde_json::Map::new()));
            if let Some(sub_map) = entry.as_object_mut() {
                insert_nested(sub_map, &key_parts[1..], value);
            }
        }
    }

    for (key, value) in map {
        let key_parts: Vec<&str> = key.split('.').collect();
        insert_nested(&mut json_map, &key_parts, value);
    }

    // JSON形式に変換してインデント付きで出力
    let json_output: String = serde_json::to_string_pretty(&json!(json_map)).unwrap();
    println!("{}", json_output);
}
