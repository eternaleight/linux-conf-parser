use rustc_hash::FxHashMap;
use std::{
    env,
    fs::File,
    io::{self, Error, Write},
};

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
            // println!("エラーが発生しましたが、空の型定義を含むファイルを出力します。");
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
            // println!("パースに失敗しました");
            Ok(())
        }
    }
}

/// 指定されたキーを空の値としてファイルに出力
pub fn output_empty_values_to_file(
    result_map: &FxHashMap<String, String>,
    output_file_path: &str,
) -> io::Result<()> {
    // 出力先ファイルを開く
    let output_file: Result<File, Error> = File::create(output_file_path);
    match output_file {
        Ok(mut file) => {
            println!(
                "\n空の型定義ファイル {} を作成しました。🖋️✨
1.schema.txtに名前を変更して型定義ファイルを作成して下さい。
2.cargo runで.conf ファイルの設定をJSON 形式で出力し、型の検証結果も表示。
",
                output_file_path
            );
            // パース結果のキーを空の値として出力
            for key in result_map.keys() {
                writeln!(file, "{} ->", key)?;
            }
            // println!("ファイルに書き込みが完了しました: {}", output_file_path);
        }
        Err(e) => {
            eprintln!("ファイル {} の作成に失敗しました: {}", output_file_path, e);
            return Err(e);
        }
    }
    Ok(())
}
