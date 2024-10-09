use serde_json::{Map, Value};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn parse_sysctl_conf(file_path: &str) -> io::Result<Map<String, Value>> {
    let path = Path::new(file_path);
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    let mut map: Map<String, Value> = Map::new(); // HashMap ではなく Map を使用

    for line in reader.lines() {
        let line = line?;
        let trimmed_line = line.trim();

        // コメント行(#, ;)や空行を無視
        if trimmed_line.is_empty() || trimmed_line.starts_with('#') || trimmed_line.starts_with(';')
        {
            continue;
        }

        // 先頭に - がある場合、エラーを無視する設定として処理
        let ignore_errors = trimmed_line.starts_with('-');
        let processed_line = if ignore_errors {
            &trimmed_line[1..].trim()
        } else {
            trimmed_line
        };

        // key=value の形式を処理
        if let Some((key, value)) = processed_line.split_once('=') {
            let key = key.trim();
            let value = value.trim();

            // 値が4096文字を超える場合は常にエラー
            if value.len() > 4096 {
                eprintln!("値が4096文字を超えています: {}", value);
                return Err(io::Error::new(io::ErrorKind::InvalidData, "値が長すぎます"));
            }

            insert_nested_key(&mut map, key, value);
        }
    }

    Ok(map)
}

fn insert_nested_key(map: &mut Map<String, Value>, key: &str, value: &str) {
    let keys: Vec<&str> = key.split('.').collect();
    let mut current_map = map;

    for (i, part) in keys.iter().enumerate() {
        if i == keys.len() - 1 {
            current_map.insert(part.to_string(), Value::String(value.to_string()));
        } else {
            current_map = current_map
                .entry(part.to_string())
                .or_insert_with(|| Value::Object(Map::new()))
                .as_object_mut()
                .unwrap();
        }
    }
}

fn main() -> io::Result<()> {
    let config = parse_sysctl_conf("config/sysctl.conf")?;
    println!("{}", serde_json::to_string_pretty(&config).unwrap());
    Ok(())
}
