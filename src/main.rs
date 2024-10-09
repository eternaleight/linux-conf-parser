use serde_json::{Map, Value};
use std::fs::{self, File};
use std::io::{self, BufRead};
use std::path::Path;

// 個々のsysctl.confファイルを解析する関数
fn parse_sysctl_conf(file_path: &Path) -> io::Result<Map<String, Value>> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let mut map: Map<String, Value> = Map::new();

    for line in reader.lines() {
        let line = line?;
        let trimmed_line = line.trim();

        // コメント行や空行を無視
        if trimmed_line.is_empty() || trimmed_line.starts_with('#') || trimmed_line.starts_with(';')
        {
            continue;
        }

        // 行が'-'で始まる場合はエラーを無視
        let ignore_errors = trimmed_line.starts_with('-');
        let processed_line = if ignore_errors {
            &trimmed_line[1..].trim()
        } else {
            trimmed_line
        };

        // key=value形式を処理
        if let Some((key, value)) = processed_line.split_once('=') {
            let key = key.trim();
            let value = value.trim();

            // 値が4096文字を超える場合はエラー
            if value.len() > 4096 {
                eprintln!("値が4096文字を超えています: {}", value);
                return Err(io::Error::new(io::ErrorKind::InvalidData, "値が長すぎます"));
            }

            insert_nested_key(&mut map, key, value);
        }
    }

    Ok(map)
}

// ネストされたキーをMapに挿入する関数
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

// ディレクトリ内の全ての.confファイルを再帰的に読み込む関数
fn parse_all_sysctl_files(directory_path: &str) -> io::Result<Map<String, Value>> {
    let mut combined_map: Map<String, Value> = Map::new();

    // ディレクトリ内のすべてのファイルを再帰的に処理
    fn read_dir_recursive(dir: &Path, combined_map: &mut Map<String, Value>) -> io::Result<()> {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            // ファイルの場合、.confファイルのみを処理
            if path.is_file() && path.extension().and_then(|ext| ext.to_str()) == Some("conf") {
                let conf_map = parse_sysctl_conf(&path)?;
                combined_map.extend(conf_map);
            }
            // ディレクトリの場合、再帰的に処理
            else if path.is_dir() {
                read_dir_recursive(&path, combined_map)?;
            }
        }
        Ok(())
    }

    let base_path = Path::new(directory_path);
    read_dir_recursive(base_path, &mut combined_map)?;

    Ok(combined_map)
}

fn main() -> io::Result<()> {
    // configディレクトリ内のすべての.confファイルを再帰的に読み込む
    let config = parse_all_sysctl_files("config")?;
    println!("{}", serde_json::to_string_pretty(&config).unwrap());
    Ok(())
}
