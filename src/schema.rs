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
    config_map: &FxHashMap<String, String>,
    schema: &FxHashMap<String, String>,
) -> Result<(), String> {
    let mut errors = Vec::new(); // エラーを収集するベクター// let mut errors = vec![]; // こっちのが楽か

    for (key, value) in config_map {
        if let Some(expected_type) = schema.get(key) {
            // 値の型を検証
            match expected_type.as_str() {
                "string" => {
                    if value.is_empty() {
                        errors.push(format!(
                            "Error: キー '{}' の値 '{}' は空の文字列です。",
                            key, value
                        ));
                    }
                }
                "bool" => {
                    if value != "true" && value != "false" {
                        errors.push(format!(
                            "\x1b[31mError: キー '{}' の値 '{}' はブール値ではありません。\x1b[0m",
                            key, value
                        ));
                    }
                }
                "int" => {
                    if value.parse::<i64>().is_err() {
                        errors.push(format!(
                            "\x1b[31mError: キー '{}' の値 '{}' は整数ではありません。\x1b[0m",
                            key, value
                        ));
                    }
                }
                _ => {
                    errors.push(format!(
                        "\x1b[31mError: キー '{}' のスキーマ型 '{}' はサポートされていません。\x1b[0m",
                        key, expected_type
                    ));
                }
            }
        } else {
            errors.push(format!(
                "\x1b[31mError: キー '{}' はスキーマに存在しません。\x1b[0m",
                key
            ));
        }
    }

    if errors.is_empty() {
        Ok(()) // エラーがない場合は正常
    } else {
        Err(errors.join("\n")) // エラーがある場合はすべてを連結して返す
    }
}
