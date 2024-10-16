use rustc_hash::FxHashMap;
use serde_json::{json, Value};

/// FxHashMapの内容を出力
pub fn _display_map(map: &FxHashMap<String, FxHashMap<String, String>>) {
    for (key, sub_map) in map {
        println!("{}", key);
        for (sub_key, value) in sub_map {
            println!("  {} {}", sub_key, value);
        }
    }
}

/// FxHashMapの内容をフラットに出力
pub fn _display_flat_map(map: &FxHashMap<String, String>) {
    for (key, value) in map {
        println!("{} {}", key, value);
    }
}

/// FxHashMapの内容をJSON形式出力（ネスト対応、整形出力）
pub fn display_json_map(map: &FxHashMap<String, String>) {
    let mut json_map: FxHashMap<String, Value> = FxHashMap::default();

    for (key, value) in map {
        let key_parts: Vec<&str> = key.split('.').collect();
        if key_parts.len() > 1 {
            // ネストされたキーがある場合
            let root_key: &str = key_parts[0];
            let nested_key: String = key_parts[1..].join(".");

            if let Some(entry) = json_map.get_mut(root_key) {
                if entry.is_object() {
                    // 既存のルートキーにネストされた値を追加
                    let entry_map: &mut serde_json::Map<String, Value> =
                        entry.as_object_mut().unwrap();
                    entry_map.insert(nested_key.to_string(), json!(value));
                }
            } else {
                // 新しいネストされたキーを作成
                let mut nested_map: FxHashMap<String, String> = FxHashMap::default();
                nested_map.insert(nested_key.to_string(), value.clone());
                json_map.insert(root_key.to_string(), json!(nested_map));
            }
        } else {
            // ネストされていないキーの場合
            json_map.insert(key.clone(), json!(value));
        }
    }

    // JSON形式に変換してインデント付きで出力
    let json_output: String = serde_json::to_string_pretty(&json!(json_map)).unwrap();
    println!("{}", json_output);
}
