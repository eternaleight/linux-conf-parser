use rustc_hash::{FxHashMap, FxHashSet};
use std::fs;
use std::io;
use std::path::Path;
use std::path::PathBuf;

use super::file_parser::parse_conf_file;
use super::schema::validate_against_schema;

pub fn parse_all_sysctl_files(
    directories: &[&str],
    schema: &FxHashMap<String, String>,
    result_map: &mut FxHashMap<String, String>,
) -> io::Result<()> {
    let mut parsed_files: FxHashSet<String> = FxHashSet::default();
    let mut all_errors: Vec<String> = Vec::new(); // 全てのエラーを収集
    for dir in directories {
        let path: &Path = Path::new(dir);
        if !path.is_dir() {
            eprintln!(
                "Error: 指定されたディレクトリ '{}' が存在しません。",
                path.display()
            );
            continue;
        }
        if let Err(e) = parse_sysctl_dir(path, &mut parsed_files, result_map) {
            all_errors.push(format!(
                "ディレクトリ '{}' のパースに失敗しました: {}",
                path.display(),
                e
            ));
        }
    }

    // パース結果をスキーマに基づいて検証
    if let Err(validation_error) = validate_against_schema(result_map, schema) {
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
    let entries: fs::ReadDir = fs::read_dir(path).map_err(|e| {
        eprintln!(
            "Error: ディレクトリ '{}' の読み込みに失敗しました: {}",
            path.display(),
            e
        );
        e
    })?;

    for entry in entries {
        let entry: fs::DirEntry = entry.map_err(|e| {
            eprintln!(
                "Error: ディレクトリ内のエントリへのアクセスに失敗しました: {}",
                e
            );
            e
        })?;
        let path: PathBuf = entry.path();

        // ファイルのパスを文字列に変換
        if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("conf") {
            parse_conf_file(&path, parsed_files, result_map)?;
        } else if path.is_dir() {
            parse_sysctl_dir(&path, parsed_files, result_map)?;
        }
    }

    Ok(())
}
