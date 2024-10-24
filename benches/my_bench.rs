#![feature(test)]

extern crate test;
use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;
use test::Bencher;

fn setup_test_file(file_name: &str, content: &str) -> PathBuf {
    let test_dir: PathBuf = PathBuf::from("test_data");
    let file_path: PathBuf = test_dir.join(file_name);

    // ファイルを含むディレクトリ全体を再帰的に作成
    if let Some(parent_dir) = file_path.parent() {
        fs::create_dir_all(parent_dir).unwrap();
    }

    // 既にファイルが存在している場合は削除
    if file_path.exists() {
        fs::remove_file(&file_path).unwrap();
    }

    // ファイルを作成して内容を書き込む
    let mut file: File = File::create(&file_path).unwrap();
    file.write_all(content.as_bytes()).unwrap();
    file.flush().unwrap(); // 明示的にフラッシュして、データを確実にディスクに書き込む

    println!("ファイル作成: {:?}", file_path);
    assert!(
        file_path.exists(),
        "ファイルが作成されていません: {:?}",
        file_path
    );

    file_path
}

// ベンチマーク用マクロ
macro_rules! create_bench {
    ($name:ident, $func:expr) => {
        #[bench]
        fn $name(b: &mut Bencher) {
            b.iter(|| {
                $func();
            });
            fs::remove_dir_all("test_data").unwrap_or_else(|_| {
                eprintln!("Error: テストデータのクリーンアップに失敗しました。");
            });
        }
    };
}

// ベンチマーク関数を生成
#[cfg(test)]
mod benchmarks {
    use super::*;
    use linux_conf_parser::core::{
        directory_parser::DirectoryParser, file_parser, schema::LoadSchema, ParseFiles,
        SchemaLoader,
    };
    use rustc_hash::FxHashMap;
    use std::path::Path;

    // 既存のベンチマーク
    create_bench!(bench_parse_conf_to_map, || {
        let file_path = setup_test_file(
            "test.conf",
            "net.ipv4.tcp_syncookies = 1\nfs.file-max = 2097152",
        );
        file_parser::parse_conf_to_map(&file_path).unwrap();
    });

    create_bench!(bench_empty_conf_file, || {
        let file_path = setup_test_file("empty.conf", ""); // 空の設定ファイルをパースする
        file_parser::parse_conf_to_map(&file_path).unwrap();
    });

    create_bench!(bench_large_conf_file, || {
        // 大量のデータを持つ設定ファイルのベンチマーク
        let large_content = "key1 = value1\n".repeat(1000); // 1000行の設定
        let file_path = setup_test_file("large.conf", &large_content);
        file_parser::parse_conf_to_map(&file_path).unwrap();
    });

    create_bench!(bench_parse_all_conf_files, || {
        let _ = setup_test_file("dir1/test1.conf", "net.ipv4.tcp_syncookies = 1");
        let _ = setup_test_file("dir1/subdir/test2.conf", "fs.file-max = 2097152");
        let directories = ["test_data/dir1"];

        // スキーマファイルを読み込む
        let schema_path = Path::new("schema.txt");
        let schema_loader = LoadSchema;
        let schema = schema_loader.load_schema(schema_path).unwrap(); // スキーマを読み込む

        let mut result_map = FxHashMap::default();
        let parser = DirectoryParser;

        parser
            .parse_all_conf_files(&directories, &schema, &mut result_map)
            .unwrap(); // トレイトメソッドを呼び出し
    });

    // 高負荷ベンチマーク
    // create_bench!(bench_parse_conf_to_map, || {
    //     // 1万行の設定ファイルを生成
    //     let large_content =
    //         "net.ipv4.tcp_syncookies = 1\n".repeat(10000) + "fs.file-max = 2097152\n";
    //     let file_path = setup_test_file("large_test.conf", &large_content);
    //     parser::parse_conf_to_map(&file_path).unwrap();
    // });

    // create_bench!(bench_parse_all_conf_files, || {
    //     // 10階層のディレクトリ構造に各100行のファイルを作成
    //     for i in 0..10 {
    //         let content = "net.ipv4.tcp_syncookies = 1\n".repeat(100);
    //         let _ = setup_test_file(&format!("dir1/level{}/test{}.conf", i, i), &content);
    //     }
    //     let directories = ["test_data/dir1"];
    //     parser::parse_all_conf_files(&directories).unwrap();
    // });

    // create_bench!(bench_empty_conf_file, || {
    //     // 100個の空ファイルを生成してパースする
    //     for i in 0..100 {
    //         let file_path = setup_test_file(&format!("empty_files/empty{}.conf", i), "");
    //         parser::parse_conf_to_map(&file_path).unwrap();
    //     }
    // });

    // create_bench!(bench_large_conf_file, || {
    //     // 1万行のデータを持つ設定ファイルを生成
    //     let large_content = "key1 = value1\n".repeat(10000); // 1万行の設定
    //     let file_path = setup_test_file("large_1m.conf", &large_content);
    //     parser::parse_conf_to_map(&file_path).unwrap();
    // });
}
