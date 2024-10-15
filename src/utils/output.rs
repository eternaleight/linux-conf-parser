use crate::core::file_parser::output_empty_values_to_file;
use rustc_hash::FxHashMap;
use std::{env, io};

/// コマンドライン引数に応じて出力方法を分岐
pub fn handle_output(
    result: Result<(), io::Error>,
    result_map: &FxHashMap<String, String>,
) -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 && args[1] == "output" {
        // 出力ファイルパスの指定
        let output_file_path = "output.txt";

        // パースの成否にかかわらず情報をファイルに出力
        if result.is_ok() {
            println!("パース結果をファイルに出力します。");
        } else {
            println!("エラーが発生しましたが、空の値を含むファイルを出力します。");
        }
        // パース結果を使ってファイルに出力
        output_empty_values_to_file(result_map, output_file_path)
    } else {
        // 標準出力
        if result.is_ok() {
            // デバッグ用
            // println!("パース結果:");
            // for (key, value) in result_map {
            //     println!("{}: {}", key, value);
            // }
            Ok(())
        } else {
            println!("パースに失敗しました");
            Ok(())
        }
    }
}
