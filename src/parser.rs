use crate::utils::display_map;
use rustc_hash::FxHashMap;
use std::fs;
use std::io::{self, BufRead};
use std::path::Path;

pub const MAX_VALUE_LENGTH: usize = 4096;

/// 設定ファイルをパースし、結果をFxHashMapに格納
pub fn parse_sysctl_conf(
    file_path: &Path,
) -> io::Result<FxHashMap<String, FxHashMap<String, String>>> {
    let file = fs::File::open(file_path).map_err(|e| {
        eprintln!(
            "Error: ファイル '{}' を開く際にエラーが発生しました: {}",
            file_path.display(),
            e
        );
        e
    })?;
    let reader = io::BufReader::new(file);

    let mut map = FxHashMap::default();

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

        // エラーハンドリングを無視する行（'-'で始まる行）
        let ignore_error = trimmed.starts_with('-');

        // '='で分割してキーと値を抽出
        if let Some((key, value)) = trimmed.split_once('=') {
            let key = key.trim();
            let value = value.trim();

            // 値が4096文字を超えた場合はエラーを出力してパニック
            if value.len() > MAX_VALUE_LENGTH {
                panic!("Error: キー '{}' の値が4096文字を超えています。👀", key);
            }

            if ignore_error {
                println!("Warning: 設定 '{}' のエラーを無視します。", key);
                continue;
            }

            insert_nested_key(&mut map, key, value);
        }
    }

    Ok(map)
}

/// ネストされたキーをFxHashMapに挿入
pub fn insert_nested_key(
    map: &mut FxHashMap<String, FxHashMap<String, String>>,
    key: &str,
    value: &str,
) {
    let mut keys = key.split('.').collect::<Vec<&str>>();

    if keys.len() == 1 {
        // ドットで区切られていない場合、単純なキーを挿入
        map.entry(key.to_string())
            .or_default()
            .insert(key.to_string(), value.to_string());
    } else {
        // ドットで区切られている場合、ネストされたマップを生成
        let first_key = keys.remove(0).to_string();
        let last_key = keys.pop().unwrap().to_string();

        let sub_map: &mut FxHashMap<String, String> = map.entry(first_key).or_default();
        sub_map.insert(last_key, value.to_string());
    }
}

/// 再帰的に指定されたディレクトリ内のすべての.confファイルをパース
pub fn parse_all_sysctl_files(directories: &[&str]) -> io::Result<()> {
    for dir in directories {
        let path = Path::new(dir);
        if path.is_dir() {
            // ディレクトリ内の.confファイルを再帰的に探索
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

                if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("conf") {
                    println!("File: {:?}", path);
                    let config_map = parse_sysctl_conf(&path)?;
                    display_map(&config_map);
                } else if path.is_dir() {
                    // サブディレクトリを再帰的に探索
                    parse_all_sysctl_files(&[path.to_str().unwrap()]).map_err(|e| {
                        eprintln!(
                            "Error: サブディレクトリ '{}' の読み込みに失敗しました: {}",
                            path.display(),
                            e
                        );
                        e
                    })?;
                }
            }
        } else {
            eprintln!(
                "Error: 指定されたディレクトリ '{}' が存在しません。",
                path.display()
            );
        }
    }
    Ok(())
}
