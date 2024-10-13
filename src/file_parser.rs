use rustc_hash::FxHashMap;
use std::fs;
use std::io::{self, BufRead};
use std::path::Path;

pub const MAX_VALUE_LENGTH: usize = 4096;

/// 設定ファイルをパースし、結果をFxHashMap格納
pub fn parse_sysctl_conf(file_path: &Path) -> io::Result<FxHashMap<String, String>> {
    let file = fs::File::open(file_path).map_err(|e| {
        eprintln!(
            "Error: ファイル '{}' を開く際にエラーが発生しました: {}",
            file_path.display(),
            e
        );
        e
    })?;
    let reader = io::BufReader::new(file);

    let mut map: FxHashMap<String, String> = FxHashMap::default();

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
            map.insert(key.to_string(), value.to_string());
        }
    }

    Ok(map)
}
