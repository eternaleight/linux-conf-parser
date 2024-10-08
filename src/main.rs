use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use serde_json::{Map, Value}; // ここでMapをインポート

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
    map: &mut Map<String, Value>, // serde_json::Map を使用
    keys: &[&str],
    value: Value,
) {
    let mut current = map;

    for &key in &keys[..keys.len() - 1] {
        current = current
            .entry(key.to_string())
            .or_insert_with(|| Value::Object(Map::new())) // serde_json::Map に合わせる
            .as_object_mut()
            .unwrap();
    }

    current.insert(keys[keys.len() - 1].to_string(), value);
}

// 設定ファイルをパースする関数
fn parse_config(filename: &str) -> io::Result<Map<String, Value>> {
    let path = Path::new(filename);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut config: Map<String, Value> = Map::new(); // serde_json::Map を使用

    // 冗長な if let を flat_map と filter_map で置き換える
    // エラーログ出力
    reader
        .lines() // Result<String, io::Error> を返す
        .inspect(|line| {
            if let Err(ref e) = line {
                eprintln!("Error reading line: {}", e);
            }
        })
        .filter_map(Result::ok) // 成功した行だけを処理
        .filter_map(|line| parse_line(&line)) // パースできた行だけを処理
        .for_each(|(key, value)| {
            let keys: Vec<&str> = key.split('.').collect();
            set_nested_map(&mut config, &keys, Value::String(value));
        });

    Ok(config)
}

fn main() -> io::Result<()> {
    // 設定ファイルをパース
    let config = parse_config("src/sysctl.conf")?;

    // 結果を出力
    println!("{}", serde_json::to_string_pretty(&config).unwrap());

    Ok(())
}
