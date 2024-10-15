mod core;
mod utils;

use core::directory_parser;
use core::schema;
use core::validate_and_parse_sysctl;
use rustc_hash::FxHashMap;
use std::io;
use utils::output::handle_output;

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

    // コマンドライン引数に応じて出力方法を分岐
    // cargo run .confファイルの設定を出力
    // cargo run output .confファイルの空の型定義ファイルを出力
    handle_output(result, &result_map)
}

// 本番想定ディレクトリ
// let directories = [
//     "/etc/sysctl.d",
//     "/run/sysctl.d",
//     "/usr/local/lib/sysctl.d",
//     "/usr/lib/sysctl.d",
//     "/lib/sysctl.d",
// ];
