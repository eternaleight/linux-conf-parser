pub mod directory_parser;
pub mod file_parser;
pub mod schema;

use rustc_hash::FxHashMap;
use std::{io, path::Path};

pub trait ParseFiles {
    fn parse_all_conf_files(
        &self,
        directories: &[&str],
        schema: &FxHashMap<String, String>,
        result_map: &mut FxHashMap<String, String>,
    ) -> io::Result<()>;
}

pub trait SchemaLoader {
    fn load_schema(&self, schema_file: &Path) -> io::Result<FxHashMap<String, String>>;
}

/// スキーマファイルを読み込み、ディレクトリを再帰的に探索してファイルをパースし、スキーマに基づいて型の整合性を検証
pub fn validate_schema_and_parse_files(
    schema_file: &str,
    directories: &[&str],
    parser: &impl ParseFiles,
    schema: &impl SchemaLoader,
    result_map: &mut FxHashMap<String, String>,
) -> io::Result<()> {
    let schema_path: &Path = Path::new(schema_file);

    // スキーマファイルを読み込む
    let schema: FxHashMap<String, String> = match schema.load_schema(schema_path) {
        Ok(schema) => schema,
        Err(e) => {
            eprintln!("スキーマファイルの読み込みに失敗しました: {}", e);
            return Err(e);
        }
    };

    // ディレクトリを探索し、ファイルをパースして結果を検証
    match parser.parse_all_conf_files(directories, &schema, result_map) {
        Ok(_) => {
            println!("全てのファイルが正常にパースされ、スキーマに従っています。");
            Ok(())
        }
        Err(e) => {
            eprintln!("設定ファイルのパース中にエラーが発生しました: {}", e);
            Err(e)
        }
    }
}
