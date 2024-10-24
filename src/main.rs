mod config;
mod core;
mod utils;

use core::{directory_parser::DirectoryParser, schema::LoadSchema};
use rustc_hash::FxHashMap;
use std::io;

use utils::output::handle_output;

fn main() -> io::Result<()> {
    let directories = [
        "test_config/etc/sysctl.d",
        "test_config/run/sysctl.d",
        "test_config/usr/local/lib/sysctl.d",
        "test_config/usr/lib/sysctl.d",
        "test_config/lib/sysctl.d",
        "test_config/etc",
        "test_config",
    ];

    // パース結果を格納するマップ
    let mut result_map: FxHashMap<String, String> = FxHashMap::default();

    let parser = DirectoryParser;
    let schema = LoadSchema;

    // スキーマ検証と.confファイルのパースを実行
    let result: Result<(), io::Error> = core::validate_schema_and_parse_files(
        config::Config::SCHEMA_FILE_PATH,
        &directories,
        &parser,
        &schema,
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
