use serde_json::Map;
use std::fs::{self, File};
use std::io::{self, BufRead};
use std::path::Path;

// トリム関数
fn trim(s: &str) -> String {
    s.trim().to_string()
}

// 1行をパースする関数
fn parse_line(line: &str) -> Option<(String, String, bool)> {
    let trimmed_line = trim(line);

    // コメントや空行を無視
    if trimmed_line.is_empty() || trimmed_line.starts_with('#') || trimmed_line.starts_with(';') {
        return None;
    }

    // 行が - で始まる場合、エラーを無視するフラグを立てる
    let ignore_error = trimmed_line.starts_with('-');
    let line = trimmed_line.strip_prefix('-').unwrap_or(&trimmed_line);

    // key = value の形式に分割
    if let Some(pos) = line.find('=') {
        let key = trim(&line[..pos]);
        let value = trim(&line[pos + 1..]);

        // 値が4096文字を超える場合に警告を表示
        if value.len() > 4096 {
            eprintln!("Warning: Value exceeds 4096 characters, truncating.");
            let truncated_value = value.chars().take(4096).collect::<String>();
            return Some((key, truncated_value, ignore_error));
        }

        return Some((key, value, ignore_error));
    }

    None
}

// ネストされたキーに対応する関数（マージを考慮）
fn set_nested_map(
    map: &mut Map<String, serde_json::Value>,
    keys: &[&str],
    value: serde_json::Value,
    _ignore_error: bool, // ignore_errorは現在使っていないため、プレフィックスを付与
) -> Result<(), &'static str> {
    let mut current = map;

    for &key in &keys[..keys.len() - 1] {
        current = current
            .entry(key.to_string())
            .or_insert_with(|| serde_json::Value::Object(serde_json::Map::new()))
            .as_object_mut()
            .ok_or("Failed to set nested map")?;
    }

    let last_key = keys[keys.len() - 1].to_string();

    match current.get_mut(&last_key) {
        Some(existing_value) if existing_value.is_object() && value.is_object() => {
            // 両方がオブジェクトの場合はマージ
            let existing_map = existing_value.as_object_mut().unwrap();
            let new_map = value.as_object().unwrap();
            for (k, v) in new_map {
                existing_map.insert(k.clone(), v.clone());
            }
            println!("Merged object for key: {:?}", last_key); // デバッグ用
        }
        _ => {
            // 上書きを防止するために、すでに値が存在する場合はスキップ
            if current.contains_key(&last_key) {
                println!("Key already exists, skipping: {:?}", last_key);
            } else {
                current.insert(last_key.clone(), value);
                println!("Set key: {:?} with value: {:?}", last_key, current.get(&last_key)); // デバッグ用
            }
        }
    }

    Ok(())
}

// 設定ファイルをパースする関数
fn parse_config(filename: &Path) -> io::Result<serde_json::Map<String, serde_json::Value>> {
    println!("Parsing file: {:?}", filename); // ファイル名を出力
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);

    let mut config: Map<String, serde_json::Value> = Map::new();

    reader
        .lines()
        .inspect(|result| {
            if let Err(ref e) = result {
                eprintln!("Error reading line: {}", e);
            }
        })
        .filter_map(Result::ok)
        .inspect(|line| println!("Parsing line: {}", line)) // 行を出力してデバッグ
        .filter_map(|line| parse_line(&line)) // ここで ignore_error を取得
        .for_each(|(key, value, ignore_error)| {
            let keys: Vec<&str> = key.split('.').collect();
            println!("Setting key: {:?} with value: {}", keys, value); // キーと値を出力
            if let Err(e) = set_nested_map(
                &mut config,
                &keys,
                serde_json::Value::String(value),
                ignore_error,
            ) {
                if !ignore_error {
                    eprintln!("Error setting config: {}", e);
                }
            }
        });

    Ok(config)
}

// 複数の設定ファイルをパースして統合する関数
fn parse_multiple_configs(
    paths: &[&str],
) -> io::Result<serde_json::Map<String, serde_json::Value>> {
    let mut final_config: Map<String, serde_json::Value> = Map::new();

    for path in paths {
        let path = Path::new(path);
        if path.is_dir() {
            // ディレクトリ内の *.conf ファイルをすべて処理
            for entry in fs::read_dir(path)? {
                let entry = entry?;
                let path = entry.path();
                if path.extension().and_then(|s| s.to_str()) == Some("conf") {
                    let config = parse_config(&path)?;
                    final_config.extend(config);
                }
            }
        } else if path.is_file() {
            // 単一ファイルを処理
            let config = parse_config(path)?;
            final_config.extend(config);
        }
    }

    Ok(final_config)
}

fn main() -> io::Result<()> {
    // sysctl.confファイルが格納されているディレクトリ
    let paths = vec![
        "config/sysctl.d/",
        "config/run/sysctl.d/",
        "config/usr/local/lib/sysctl.d/",
        "config/usr/lib/sysctl.d/system.conf",
        "config/lib/sysctl.d/",
        "config/sysctl.conf",
    ];

    // 指定したすべてのファイル・ディレクトリをパースして統合
    let config = parse_multiple_configs(&paths)?;

    // 結果をJSON形式で出力
    println!("{}", serde_json::to_string_pretty(&config).unwrap());

    Ok(())
}
