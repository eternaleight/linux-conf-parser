#![feature(test)]

extern crate test;
use projects::parser;
use std::fs::{self, File};
use std::io::Write;
use test::Bencher;

fn setup_test_file(file_name: &str, content: &str) -> std::path::PathBuf {
    let test_dir = std::path::PathBuf::from("test_data");
    let file_path = test_dir.join(file_name);

    if let Some(parent_dir) = file_path.parent() {
        fs::create_dir_all(parent_dir).unwrap();
    }

    let mut file = File::create(&file_path).unwrap();
    file.write_all(content.as_bytes()).unwrap();
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
            // クリーンアップ処理を追加
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

    // 既存のベンチマーク
    create_bench!(bench_parse_sysctl_conf, || {
        let file_path = setup_test_file(
            "test.conf",
            "net.ipv4.tcp_syncookies = 1\nfs.file-max = 2097152",
        );
        parser::parse_sysctl_conf(&file_path).unwrap();
    });

    create_bench!(bench_parse_all_sysctl_files, || {
        let _ = setup_test_file("dir1/test1.conf", "net.ipv4.tcp_syncookies = 1");
        let _ = setup_test_file("dir1/subdir/test2.conf", "fs.file-max = 2097152");
        let directories = ["test_data/dir1"];
        parser::parse_all_sysctl_files(&directories).unwrap();
    });

    create_bench!(bench_empty_conf_file, || {
        let file_path = setup_test_file("empty.conf", ""); // 空の設定ファイルをパースする
        parser::parse_sysctl_conf(&file_path).unwrap();
    });

    create_bench!(bench_large_conf_file, || {
        // 大量のデータを持つ設定ファイルのベンチマーク
        let large_content = "key1 = value1\n".repeat(1000); // 1000行の設定
        let file_path = setup_test_file("large.conf", &large_content);
        parser::parse_sysctl_conf(&file_path).unwrap();
    });

    // 高負荷ベンチマーク
    // create_bench!(bench_parse_sysctl_conf, || {
    //     // 1万行の設定ファイルを生成
    //     let large_content =
    //         "net.ipv4.tcp_syncookies = 1\n".repeat(10000) + "fs.file-max = 2097152\n";
    //     let file_path = setup_test_file("large_test.conf", &large_content);
    //     parser::parse_sysctl_conf(&file_path).unwrap();
    // });

    // create_bench!(bench_parse_all_sysctl_files, || {
    //     // 10階層のディレクトリ構造に各100行のファイルを作成
    //     for i in 0..10 {
    //         let content = "net.ipv4.tcp_syncookies = 1\n".repeat(100);
    //         let _ = setup_test_file(&format!("dir1/level{}/test{}.conf", i, i), &content);
    //     }
    //     let directories = ["test_data/dir1"];
    //     parser::parse_all_sysctl_files(&directories).unwrap();
    // });

    // create_bench!(bench_empty_conf_file, || {
    //     // 100個の空ファイルを生成してパースする
    //     for i in 0..100 {
    //         let file_path = setup_test_file(&format!("empty_files/empty{}.conf", i), "");
    //         parser::parse_sysctl_conf(&file_path).unwrap();
    //     }
    // });

    // create_bench!(bench_large_conf_file, || {
    //     // 1万行のデータを持つ設定ファイルを生成
    //     let large_content = "key1 = value1\n".repeat(10000); // 1万行の設定
    //     let file_path = setup_test_file("large_1m.conf", &large_content);
    //     parser::parse_sysctl_conf(&file_path).unwrap();
    // });
}
