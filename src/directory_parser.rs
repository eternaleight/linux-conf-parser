use crate::file_parser::parse_sysctl_conf;
use crate::schema::validate_against_schema;
use crate::utils::display_json_map;
use rustc_hash::{FxHashMap, FxHashSet};
use std::fs;
use std::io;
use std::path::Path;

pub fn parse_all_sysctl_files(
    directories: &[&str],
    schema: &FxHashMap<String, String>,
) -> io::Result<()> {
    let mut parsed_files = FxHashSet::default();
    let mut result_map = FxHashMap::default();
    let mut all_errors = Vec::new(); // 全てのエラーを収集

    for dir in directories {
        let path = Path::new(dir);
        if !path.is_dir() {
            eprintln!(
                "Error: 指定されたディレクトリ '{}' が存在しません。",
                path.display()
            );
            continue;
        }
        parse_sysctl_dir(path, &mut parsed_files, &mut result_map)?;
    }

    // パース結果をスキーマに基づいて検証
    if let Err(validation_error) = validate_against_schema(&result_map, schema) {
        all_errors.push(validation_error); // エラーを収集
    }

    // すべてのエラーを出力
    if !all_errors.is_empty() {
        for error in all_errors {
            eprintln!("{}", error);
        }
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "設定ファイルにエラーがあります。",
        ));
    }
    Ok(())
}

/// 再帰的にディレクトリ内の.confファイルを探索してパース
fn parse_sysctl_dir(
    path: &Path,
    parsed_files: &mut FxHashSet<String>,
    result_map: &mut FxHashMap<String, String>,
) -> io::Result<()> {
    for entry in fs::read_dir(path).map_err(|e| {
        eprintln!(
            "Error: ディレクトリ '{}' の読み込みに失敗しました: {}",
            path.display(),
            e
        );
        e
    })? {
        let entry = entry.map_err(|e| {
            eprintln!(
                "Error: ディレクトリ内のエントリへのアクセスに失敗しました: {}",
                e
            );
            e
        })?;
        let path = entry.path();

        // ファイルのパスを文字列に変換
        let path_str = path.to_string_lossy().to_string();

        if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("conf") {
            if parsed_files.contains(&path_str) {
                // 既にパース済みならスキップ
                continue;
            }
            println!("File: {:?}", path);
            let config_map = parse_sysctl_conf(&path)?;
            display_json_map(&config_map);
            println!();

            for (key, value) in &config_map {
                result_map.insert(key.to_string(), value.clone());
            }

            // パース済みとしてセットに追加
            parsed_files.insert(path_str);
        } else if path.is_dir() {
            parse_sysctl_dir(&path, parsed_files, result_map)?;
        }
    }
    Ok(())
}
