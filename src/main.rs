mod directory_parser;
mod file_parser;
mod schema;
mod utils;

use std::{io, path::Path};

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

    // スキーマファイルを読み込む
    let schema_path = Path::new("schema.txt");
    match schema::load_schema(schema_path) {
        Ok(schema) => {
            // スキーマのロードに成功した場合、ディレクトリを再帰的に探索してファイルをパース
            match directory_parser::parse_all_sysctl_files(&directories, &schema) {
                Ok(_) => {
                    println!("全てのファイルが正常にパースされ、スキーマに従っています。");
                }
                Err(e) => {
                    eprintln!("設定ファイルのパース中にエラーが発生しました: {}", e);
                    return Err(e);
                }
            }
        }
        Err(e) => {
            eprintln!("スキーマファイルの読み込みに失敗しました: {}", e);
            return Err(e);
        }
    }

    Ok(())
}

// 本番システム用
// mod parser;
// mod utils;

// use std::io;

// fn main() -> io::Result<()> {
//     // 再帰的に探索するディレクトリ
//     let directories = [
//         "/etc/sysctl.d",
//         "/run/sysctl.d",
//         "/usr/local/lib/sysctl.d",
//         "/usr/lib/sysctl.d",
//         "/lib/sysctl.d",
//         "/etc",
//     ];

//     parser::parse_all_sysctl_files(&directories)?;

//     Ok(())
// }
