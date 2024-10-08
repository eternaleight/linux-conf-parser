use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// 文字列の前後の空白を削除する関数
fn trim(s: &str) -> String {
    s.trim().to_string()
}

// 1行をパースする関数
fn parse_line(line: &str) -> Option<(String, String)> {
    let trimmed_line = trim(line);

    // コメントや空行を無視
    if trimmed_line.is_empty() || trimmed_line.starts_with('#') || trimmed_line.starts_with(';') {
        return None;
    }

    // key = value の形式に分割
    if let Some(pos) = trimmed_line.find('=') {
        let key = trim(&trimmed_line[..pos]);
        let value = trim(&trimmed_line[pos + 1..]);
        return Some((key, value));
    }

    None
}

// ネストされたキーに対応する関数
fn set_nested_map(
    map: &mut HashMap<String, serde_json::Value>,
    keys: &[&str],
    value: serde_json::Value,
) {
    let mut current = map;

    for &key in &keys[..keys.len() - 1] {
        current = current
            .entry(key.to_string())
            .or_insert_with(|| serde_json::Value::Object(HashMap::new()))
            .as_object_mut()
            .unwrap();
    }

    current.insert(keys[keys.len() - 1].to_string(), value);
}

// 設定ファイルをパースする関数
fn parse_config(filename: &str) -> io::Result<HashMap<String, serde_json::Value>> {
    let path = Path::new(filename);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut config: HashMap<String, serde_json::Value> = HashMap::new();

    for line in reader.lines() {
        if let Ok(line) = line {
            if let Some((key, value)) = parse_line(&line) {
                let keys: Vec<&str> = key.split('.').collect();
                set_nested_map(&mut config, &keys, serde_json::Value::String(value));
            }
        }
    }

    Ok(config)
}

fn main() -> io::Result<()> {
    // 設定ファイルをパース
    let config = parse_config("sysctl.conf")?;

    // 結果を出力
    println!("{}", serde_json::to_string_pretty(&config).unwrap());

    Ok(())
}
