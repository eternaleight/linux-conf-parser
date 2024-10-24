use rustc_hash::{FxHashMap, FxHashSet};
use std::fs::{self, File};
use std::io::{self, BufRead, Error};
use std::path::Path;

use crate::config::Config;
use crate::utils::display::display_json_map;

/// .confファイルのパース処理
pub fn parse_conf_file(
    path: &Path,
    parsed_files: &mut FxHashSet<String>,
    result_map: &mut FxHashMap<String, String>,
) -> io::Result<()> {
    let path_str: String = path.to_string_lossy().to_string();

    if parsed_files.contains(&path_str) {
        // 既にパース済みならスキップ
        return Ok(());
    }

    println!("File: {:?}", path);
    match parse_conf_to_map(path) {
        Ok(config_map) => {
            display_json_map(&config_map);
            println!();

            // パース結果をresult_mapに追加
            for (key, value) in config_map {
                result_map.insert(key.to_string(), value);
            }

            // パース済みとしてセットに追加
            parsed_files.insert(path_str);
        }
        Err(e) => {
            eprintln!(
                "Error: ファイル '{}' のパースに失敗しました: {}",
                path.display(),
                e
            );
        }
    }

    Ok(())
}

/// 設定ファイルをパースし、結果をFxHashMap格納
pub fn parse_conf_to_map(file_path: &Path) -> io::Result<FxHashMap<String, String>> {
    let file: File = fs::File::open(file_path).map_err(|e: Error| {
        eprintln!(
            "Error: ファイル '{}' を開く際にエラーが発生しました: {}",
            file_path.display(),
            e
        );
        e
    })?;
    let reader: io::BufReader<File> = io::BufReader::new(file);

    let mut map: FxHashMap<String, String> = FxHashMap::default();

    for line in reader.lines() {
        let line: String = line.map_err(|e: Error| {
            eprintln!(
                "Error: ファイル '{}' の読み込み中にエラーが発生しました: {}",
                file_path.display(),
                e
            );
            e
        })?;
        let trimmed: &str = line.trim();

        // 空行とコメント行を無視
        if trimmed.is_empty() || trimmed.starts_with('#') || trimmed.starts_with(';') {
            continue;
        }

        // '='で分割してキーと値を抽出
        if let Some((key, value)) = trimmed.split_once('=') {
            let key: &str = key.trim();
            let value: &str = value.trim();

            // 値が4096文字を超えた場合はパニック
            if value.len() > Config::MAX_VALUE_LENGTH {
                panic!("Error: キー '{}' の値が4096文字を超えています。👀", key);
            }
            map.insert(key.to_string(), value.to_string());
        }
    }

    Ok(map)
}
