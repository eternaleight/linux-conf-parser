use rustc_hash::FxHashMap;
use std::fs;
use std::io::{self, BufRead};
use std::path::Path;

/// スキーマファイルを読み込み、キーと型のペアを返す
pub fn load_schema(file_path: &Path) -> io::Result<FxHashMap<String, String>> {
    let file = fs::File::open(file_path).map_err(|e| {
        eprintln!(
            "Error: スキーマファイル '{}' を開く際にエラーが発生しました: {}",
            file_path.display(),
            e
        );
        e
    })?;
    let reader = io::BufReader::new(file);
    let mut schema = FxHashMap::default();

    for line in reader.lines() {
        let line = line.map_err(|e| {
            eprintln!(
                "Error: スキーマファイル '{}' の読み込み中にエラーが発生しました: {}",
                file_path.display(),
                e
            );
            e
        })?;
        let trimmed = line.trim();

        // 空行やコメント行を無視
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }

        // "->" で分割してキーと型を抽出
        if let Some((key, value_type)) = trimmed.split_once("->") {
            let key = key.trim().to_string();
            let value_type = value_type.trim().to_string();
            schema.insert(key, value_type);
        }
    }

    Ok(schema)
}

/// 設定ファイルの内容をスキーマと照合して検証
pub fn validate_against_schema(
    config_map: &FxHashMap<String, FxHashMap<String, String>>,
    schema: &FxHashMap<String, String>,
) -> Result<(), String> {
    for (key, value_map) in config_map {
        for (sub_key, value) in value_map {
            let full_key = format!("{}.{}", key, sub_key);
            if let Some(expected_type) = schema.get(&full_key) {
                // 値の型を検証
                match expected_type.as_str() {
                    "string" => { /* 基本的にすべての値は文字列として扱われる */
                    }
                    "bool" => {
                        if value != "true" && value != "false" {
                            return Err(format!(
                                "Error: キー '{}' の値はブール値ではありません。",
                                full_key
                            ));
                        }
                    }
                    "int" => {
                        if value.parse::<i64>().is_err() {
                            return Err(format!(
                                "Error: キー '{}' の値は整数ではありません。",
                                full_key
                            ));
                        }
                    }
                    _ => {
                        return Err(format!(
                            "Error: キー '{}' のスキーマ型 '{}' はサポートされていません。",
                            full_key, expected_type
                        ))
                    }
                }
            } else {
                return Err(format!(
                    "Error: キー '{}' はスキーマに存在しません。",
                    full_key
                ));
            }
        }
    }
    Ok(())
}
