use serde_json::Map;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn trim(s: &str) -> String {
    s.trim().to_string()
}

// 1行をパースする関数
fn parse_line(line: &str) -> Option<(String, String, bool)> {
    let trimmed_line = trim(line);

    // コメントや空行を無視
    if trimmed_line.is_empty() || trimmed_line.starts_with('#') || trimmed_line.starts_with(';') {
        return None;
    }

    // 行が - で始まる場合、エラーを無視するフラグを立てる
    let ignore_error = trimmed_line.starts_with('-');
    let line = trimmed_line.strip_prefix('-').unwrap_or(&trimmed_line);

    // key = value の形式に分割
    if let Some(pos) = line.find('=') {
        let key = trim(&line[..pos]);
        let value = trim(&line[pos + 1..]);

        // 値が4096文字を超える場合に警告を表示
        if value.len() > 4096 {
            eprintln!("Warning: Value exceeds 4096 characters, truncating.");
            let truncated_value = value.chars().take(4096).collect::<String>();
            return Some((key, truncated_value, ignore_error));
        }

        return Some((key, value, ignore_error));
    }

    None
}

// ネストされたキーに対応する関数
fn set_nested_map(
    map: &mut Map<String, serde_json::Value>,
    keys: &[&str],
    value: serde_json::Value,
    ignore_error: bool,
) -> Result<(), &'static str> {
    let mut current = map;

    for &key in &keys[..keys.len() - 1] {
        current = current
            .entry(key.to_string())
            .or_insert_with(|| serde_json::Value::Object(serde_json::Map::new()))
            .as_object_mut()
            .ok_or("Failed to set nested map")?;
    }

    // 値の挿入
    if current
        .insert(keys[keys.len() - 1].to_string(), value)
        .is_none()
    {
        Ok(())
    } else if ignore_error {
        eprintln!("Ignoring error for key: {:?}", keys);
        return Ok(());
    } else {
        return Err("Failed to insert key-value pair");
    }
}

// 設定ファイルをパースする関数
fn parse_config(filename: &str) -> io::Result<serde_json::Map<String, serde_json::Value>> {
    let path = Path::new(filename);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut config: Map<String, serde_json::Value> = Map::new();
    // let mut config: HashMap<String, serde_json::Value> = HashMap::new();

    // forループ
    // for line in reader.lines() {
    //     if let Ok(line) = line {
    //         if let Some((key, value)) = parse_line(&line) {
    //             let keys: Vec<&str> = key.split('.').collect();
    //             set_nested_map(&mut config, &keys, serde_json::Value::String(value));
    //         }
    //     }
    // }

    // forループからmap_whileに変更
    // システム全体の一貫性やクリティカルな設定が重要な場合（サービスなどでこの処理がシステム全体に深刻な影響を与える場合はエラーが発生したら止める）
    // reader
    //     .lines()
    //     .inspect(|result| {
    //         if let Err(ref e) = result {
    //             eprintln!("Error reading line: {}", e); // エラーメッセージをログに出力
    //         }
    //     })
    //     .map_while(Result::ok) // 成功した行だけを処理し、エラーが出たら処理を終了
    //     .filter_map(|line| parse_line(&line)) // パースできた行だけを処理
    //     .for_each(|(key, value)| {
    //         let keys: Vec<&str> = key.split('.').collect();
    //         set_nested_map(&mut config, &keys, serde_json::Value::String(value));
    //     });

    // map_whileからfilter_map, inspectに変更、エラーが出たら処理を終了したくないのでfilter_map(Result::ok)を使用、inspect を使って、エラーがあった時にその情報を出力しつつ、処理を進める。
    // 辞書型・Map等に格納するプログラムという使い方に関して、まずは、ファイルの柔軟な動作や非クリティカルな設定が重要だと思うので、エラーが発生しても処理を続ける選択をしました。
    reader
        .lines()
        .inspect(|result| {
            if let Err(ref e) = result {
                eprintln!("Error reading line: {}", e);
            }
        })
        .filter_map(Result::ok)
        .filter_map(|line| parse_line(&line)) // ここで ignore_error を取得
        .for_each(|(key, value, ignore_error)| {
            let keys: Vec<&str> = key.split('.').collect();
            // set_nested_mapを呼び出す際にignore_errorを追加
            if let Err(e) = set_nested_map(
                &mut config,
                &keys,
                serde_json::Value::String(value),
                ignore_error,
            ) {
                if !ignore_error {
                    eprintln!("Error setting config: {}", e);
                }
            }
        });

    Ok(config)
}

fn main() -> io::Result<()> {
    let config = parse_config("config/sysctl.conf")?;

    println!("{}", serde_json::to_string_pretty(&config).unwrap());

    Ok(())
}
