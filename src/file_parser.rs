use rustc_hash::FxHashMap;
use std::fs;
use std::io::{self, BufRead};
use std::path::Path;

pub const MAX_VALUE_LENGTH: usize = 4096;

/// 設定ファイルをパースし、結果をFxHashMapに格納
pub fn parse_sysctl_conf(
    file_path: &Path,
) -> io::Result<FxHashMap<String, FxHashMap<String, String>>> {
    let file = fs::File::open(file_path).map_err(|e| {
        eprintln!(
            "Error: ファイル '{}' を開く際にエラーが発生しました: {}",
            file_path.display(),
            e
        );
        e
    })?;
    let reader = io::BufReader::new(file);

    let mut map = FxHashMap::default();

    for line in reader.lines() {
        let line = line.map_err(|e| {
            eprintln!(
                "Error: ファイル '{}' の読み込み中にエラーが発生しました: {}",
                file_path.display(),
                e
            );
            e
        })?;
        let trimmed = line.trim();

        // 空行とコメント行を無視
        if trimmed.is_empty() || trimmed.starts_with('#') || trimmed.starts_with(';') {
            continue;
        }

        // '='で分割してキーと値を抽出
        if let Some((key, value)) = trimmed.split_once('=') {
            let key = key.trim();
            let value = value.trim();

            // 値が4096文字を超えた場合はパニック
            if value.len() > MAX_VALUE_LENGTH {
                panic!("Error: キー '{}' の値が4096文字を超えています。👀", key);
            }

            if trimmed.starts_with('-') {
                println!("Warning: 設定 '{}' のエラーを無視します。", key);
                continue;
            }

            insert_nested_key(&mut map, key, value);
        }
    }

    Ok(map)
}

/// ネストされたキーをFxHashMapに挿入
pub fn insert_nested_key(
    map: &mut FxHashMap<String, FxHashMap<String, String>>,
    key: &str,
    value: &str,
) {
    let mut keys = key.split('.').collect::<Vec<&str>>();

    if keys.len() == 1 {
        // ドットで区切られていない場合、単純なキーを挿入
        // println!("キーを挿入: {} -> {}", key, value);  // デバッグログ追加
        map.entry(key.to_string())
            .or_default()
            .insert(key.to_string(), value.to_string());
    } else {
        // ドットで区切られている場合、ネストされたマップを生成
        let first_key = keys.remove(0).to_string();
        let last_key = keys.pop().unwrap().to_string();
        // println!("ネストされたキーを挿入: {} -> {} -> {}", first_key, last_key, value);  // デバッグログ追加
        map.entry(first_key)
            .or_default()
            .insert(last_key, value.to_string());
    }
}
