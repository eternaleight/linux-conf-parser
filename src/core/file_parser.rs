use rustc_hash::FxHashMap;
use std::fs::{self, File};
use std::io::{self, BufRead, Write};
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

/// 指定されたキーを空の値としてファイルに出力
pub fn output_empty_values_to_file(
    result_map: &FxHashMap<String, String>,
    output_file_path: &str,
) -> io::Result<()> {
    // 出力先ファイルを開く
    let output_file = File::create(output_file_path);
    match output_file {
        Ok(mut file) => {
            println!("ファイル {} を作成しました。", output_file_path);
            // パース結果のキーを空の値として出力
            for key in result_map.keys() {
                writeln!(file, "{} ->", key)?;
            }
            println!("ファイルに書き込みが完了しました: {}", output_file_path);
        }
        Err(e) => {
            eprintln!("ファイル {} の作成に失敗しました: {}", output_file_path, e);
            return Err(e);
        }
    }
    Ok(())
}
