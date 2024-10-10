use core::panic;
use rustc_hash::FxHashMap;
use std::fs;
use std::io::{self, BufRead};
use std::path::Path;

const MAX_VALUE_LENGTH: usize = 4096;

/// 設定ファイルをパースし、結果をFxHashMapに格納
fn parse_sysctl_conf(file_path: &Path) -> io::Result<FxHashMap<String, FxHashMap<String, String>>> {
    let file = fs::File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let mut map = FxHashMap::default();

    for line in reader.lines() {
        let line = line?;
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

            // 値が4096文字を超えた場合はエラーメッセージを出力し、即座に終了
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
fn insert_nested_key(
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

/// FxHashMapの内容を出力
fn display_map(map: &FxHashMap<String, FxHashMap<String, String>>) {
    for (key, sub_map) in map {
        println!("{}", key);
        for (sub_key, value) in sub_map {
            println!("  {} {}", sub_key, value); // = や : なしで出力
        }
        println!(); //最後だけ改行
    }
}

/// 再帰的に指定されたディレクトリ内のすべての.confファイルをパース
fn parse_all_sysctl_files(directories: &[&str]) -> io::Result<()> {
    for dir in directories {
        let path = Path::new(dir);
        if path.is_dir() {
            // ディレクトリ内の.confファイルを再帰的に探索
            for entry in fs::read_dir(path)? {
                let entry = entry?;
                let path = entry.path();

                if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("conf") {
                    println!("File: {:?}", path);
                    let config_map = parse_sysctl_conf(&path)?;
                    display_map(&config_map);
                } else if path.is_dir() {
                    // サブディレクトリを再帰的に探索
                    parse_all_sysctl_files(&[path.to_str().unwrap()])?;
                }
            }
        }
    }
    Ok(())
}

fn main() -> io::Result<()> {
    // 再帰的に探索するディレクトリ
    let directories = [
        "config/etc/sysctl.d",
        "config/run/sysctl.d",
        "config/usr/local/lib/sysctl.d",
        "config/usr/lib/sysctl.d",
        "config/lib/sysctl.d",
        "config/etc",
        "config",
    ];

    // parse_all_sysctl_filesのエラーをキャッチして終了
    if let Err(e) = parse_all_sysctl_files(&directories) {
        eprintln!("Error: エラーが発生しました: {}", e);
    }

    Ok(())
}
