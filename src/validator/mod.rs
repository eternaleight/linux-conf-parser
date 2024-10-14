use rustc_hash::FxHashMap;
use std::io;
use std::path::Path;

/// スキーマファイルを読み込み、ディレクトリを再帰的に探索してファイルをパースし、スキーマに基づいて検証
pub fn load_and_validate_schema(
    schema_file: &str,
    directories: &[&str],
    parse_all_sysctl_files_fn: fn(&[&str], &FxHashMap<String, String>) -> io::Result<()>,
    load_schema_fn: fn(&Path) -> io::Result<FxHashMap<String, String>>,
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
    match parse_all_sysctl_files_fn(directories, &schema) {
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
