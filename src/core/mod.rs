pub mod directory_parser;
pub mod file_parser;
pub mod schema;

use rustc_hash::FxHashMap;
use std::{io, path::Path};

type ParseFn =
    fn(&[&str], &FxHashMap<String, String>, &mut FxHashMap<String, String>) -> io::Result<()>;

/// スキーマファイルを読み込み、ディレクトリを再帰的に探索してファイルをパースし、スキーマに基づいて検証
pub fn validate_and_parse_sysctl(
    schema_file: &str,
    directories: &[&str],
    parse_all_sysctl_files_fn: ParseFn,
    load_schema_fn: fn(&Path) -> io::Result<FxHashMap<String, String>>,
    result_map: &mut FxHashMap<String, String>,
) -> io::Result<()> {
    let schema_path = Path::new(schema_file);

    // スキーマファイルを読み込む
    let schema = match load_schema_fn(schema_path) {
        Ok(schema) => schema,
        Err(e) => {
            eprintln!("スキーマファイルの読み込みに失敗しました: {}", e);
            return Err(e);
        }
    };

    // ディレクトリを探索し、ファイルをパースして結果を検証
    match parse_all_sysctl_files_fn(directories, &schema, result_map) {
        Ok(_) => {
            println!("全てのファイルが正常にパースされ、スキーマに従っています。");
            Ok(())
        }
        Err(e) => {
            eprintln!("設定ファイルのパース中にエラーが発生しました: {}\n", e);
            Err(e)
        }
    }
}
