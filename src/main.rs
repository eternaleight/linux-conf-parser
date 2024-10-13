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
    let schema = schema::load_schema(schema_path)?;

    directory_parser::parse_all_sysctl_files(&directories, &schema)?;

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
