mod directory_parser;
mod file_parser;
mod schema;
mod utils;

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

    // 依存性を注入してスキーマファイルの読み込みと検証を実行
    schema::load_and_validate_schema(
        "schema.txt",
        &directories,
        directory_parser::parse_all_sysctl_files,
        schema::load_schema,
    )
}

// // 本番システム用
// mod directory_parser;
// mod file_parser;
// mod schema;
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

// // 依存性を注入してスキーマファイルの読み込みと検証を実行
// schema::load_and_validate_schema(
//     "schema.txt",
//     &directories,
//     directory_parser::parse_all_sysctl_files,
//     schema::load_schema,
// )
// }
