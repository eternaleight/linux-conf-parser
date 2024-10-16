use rustc_hash::FxHashMap;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Error};
use std::path::Path;

/// スキーマファイルを読み込み、キーと型のペアを返す
pub fn load_schema(file_path: &Path) -> io::Result<FxHashMap<String, String>> {
    let file: fs::File = fs::File::open(file_path).map_err(|e: Error| {
        eprintln!(
            "Error: スキーマファイル '{}' を開く際にエラーが発生しました: {}",
            file_path.display(),
            e
        );
        e
    })?;
    let reader: BufReader<File> = io::BufReader::new(file);
    let mut schema: FxHashMap<String, String> = FxHashMap::default();

    for line in reader.lines() {
        let line: String = line.map_err(|e: Error| {
            eprintln!(
                "Error: スキーマファイル '{}' の読み込み中にエラーが発生しました: {}",
                file_path.display(),
                e
            );
            e
        })?;
        let trimmed: &str = line.trim();

        // 空行やコメント行を無視
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }

        // "->" で分割してキーと型を抽出
        if let Some((key, value_type)) = trimmed.split_once("->") {
            let key: String = key.trim().to_string();
            let value_type: String = value_type.trim().to_string();
            schema.insert(key, value_type);
        }
    }

    Ok(schema)
}

/// 数値かどうかを判定するヘルパー関数
fn is_numeric(value: &str) -> bool {
    // 文字列が数値かどうかを判定するための正規表現
    let re: regex::Regex = regex::Regex::new(r"^-?\d+(\.\d+)?$").unwrap();
    re.is_match(value)
}

/// 設定ファイルの内容をスキーマと照合して検証
pub fn validate_against_schema(
    config_map: &FxHashMap<String, String>,
    schema: &FxHashMap<String, String>,
) -> Result<(), String> {
    let mut errors: Vec<String> = Vec::new();

    for (key, value) in config_map {
        if let Some(expected_type) = schema.get(key) {
            match expected_type.as_str() {
                "string" => validate_string(key, value, &mut errors),
                "bool" => validate_bool(key, value, &mut errors),
                "int" => validate_int(key, value, &mut errors),
                "float" => validate_float(key, value, &mut errors),
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
        Ok(())
    } else {
        Err(errors.join("\n"))
    }
}

/// 文字列の検証
fn validate_string(key: &str, value: &str, errors: &mut Vec<String>) {
    if value.is_empty() {
        let error_message: String = format!(
            "\x1b[31mError: キー '{}' の値 '{}' は空の文字列です。\x1b[0m",
            key, value
        );
        // println!("Generated Error: {}", error_message); // デバッグ出力
        errors.push(error_message);
    } else if value == "true" || value == "false" {
        let error_message: String = format!(
            "\x1b[31mError: キー '{}' の値 '{}' はブール値ではなく、文字列である必要があります。\x1b[0m",
            key, value
        );
        // println!("Generated Error: {}", error_message); // デバッグ出力
        errors.push(error_message);
    } else if is_numeric(value) {
        let error_message: String = format!(
            "\x1b[31mError: キー '{}' の値 '{}' は数値ではなく、文字列である必要があります。\x1b[0m",
            key, value
        );
        // println!("Generated Error: {}", error_message);  // デバッグ出力
        errors.push(error_message);
    }
}

/// ブール値の検証
fn validate_bool(key: &str, value: &str, errors: &mut Vec<String>) {
    if value != "true" && value != "false" {
        errors.push(format!(
            "\x1b[31mError: キー '{}' の値 '{}' はブール値ではありません。\x1b[0m",
            key, value
        ));
    }
}

/// 整数の検証
fn validate_int(key: &str, value: &str, errors: &mut Vec<String>) {
    // 小数点が含まれているかチェックし、整数としてパースできるか確認
    if value.contains('.') || value.parse::<i64>().is_err() {
        errors.push(format!(
            "\x1b[31mError: キー '{}' の値 '{}' は整数ではありません。\x1b[0m",
            key, value
        ));
    }
}

/// 浮動小数点数の検証
fn validate_float(key: &str, value: &str, errors: &mut Vec<String>) {
    if value.parse::<f64>().is_err() {
        errors.push(format!(
            "\x1b[31mError: キー '{}' の値 '{}' は浮動小数点数ではありません。\x1b[0m",
            key, value
        ));
    }
}
