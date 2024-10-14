mod core;
mod utils;

use core::directory_parser;
use core::file_parser::output_empty_values_to_file;
use core::schema;
use core::validate_and_parse_sysctl;
use rustc_hash::FxHashMap;
use std::io;

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

    // パース結果を格納するマップ
    let mut result_map: FxHashMap<String, String> = FxHashMap::default();

    // スキーマ検証とSysctlファイルのパースを実行
    let result = validate_and_parse_sysctl(
        "schema.txt",
        &directories,
        directory_parser::parse_all_sysctl_files,
        schema::load_schema,
        &mut result_map,
    );

    // デバッグのため、パース結果を表示
    if result.is_ok() {
        println!("パース結果:");
        for (key, value) in &result_map {
            println!("{}: {}", key, value);
        }
    } else {
        println!("パースに失敗しました");
    }

    // パース結果が問題ない場合は、ファイルに出力
    let output_file_path = "output.txt";
    println!("出力ファイル作成: {}", output_file_path);
    output_empty_values_to_file(&result_map, output_file_path)?; // パース結果を使ってファイルに出力

    result
}
