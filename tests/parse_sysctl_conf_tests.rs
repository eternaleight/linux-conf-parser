use projects::parser::{parse_all_sysctl_files, parse_sysctl_conf, MAX_VALUE_LENGTH};
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};

/// テスト用の一時ディレクトリとファイルを作成する関数
fn setup_test_file(file_name: &str, content: &str) -> PathBuf {
    let test_dir = PathBuf::from("test_data");
    let file_path = test_dir.join(file_name);

    // ファイルを含むディレクトリ全体を再帰的に作成
    if let Some(parent_dir) = file_path.parent() {
        fs::create_dir_all(parent_dir).unwrap();
    }

    // 既にファイルが存在している場合は削除
    if file_path.exists() {
        fs::remove_file(&file_path).unwrap();
    }

    // ファイルを作成して内容を書き込む
    let mut file = File::create(&file_path).unwrap();
    file.write_all(content.as_bytes()).unwrap();

    file_path
}

/// 存在しないファイルを開いた場合のエラーテスト
#[test]
fn test_non_existent_file() {
    let file_path = Path::new("non_existent.conf");
    let result = parse_sysctl_conf(file_path);
    assert!(result.is_err());
    if let Err(e) = result {
        assert_eq!(e.kind(), std::io::ErrorKind::NotFound);
    }
}

/// 4096文字を超える値が含まれている場合のエラーテスト
#[test]
#[should_panic(expected = "値が4096文字を超えています")]
fn test_value_too_long() {
    let long_value = "A".repeat(MAX_VALUE_LENGTH + 1);
    let content = format!("long.key = {}", long_value);
    let file_path = setup_test_file("long_value.conf", &content);

    let result = parse_sysctl_conf(&file_path);
    assert!(result.is_err());

    // エラーの種類を確認
    if let Err(e) = result {
        assert_eq!(e.kind(), std::io::ErrorKind::InvalidData);
    }
}

/// 正常な設定ファイルを読み込むテスト
#[test]
fn test_valid_conf_file() {
    let content = "net.ipv4.tcp_syncookies = 1\nfs.file-max = 2097152";
    let file_path = setup_test_file("valid.conf", content);

    let result = parse_sysctl_conf(&file_path);
    assert!(result.is_ok(), "設定ファイルのパースに失敗しました");

    let map = result.unwrap();

    // まずマップ全体を表示して、デバッグしやすくする
    println!("{:?}", map);

    assert_eq!(
        map.get("net")
            .expect("net が存在しません")
            .get("tcp_syncookies")
            .expect("tcp_syncookies が存在しません"),
        "1"
    );
    assert_eq!(
        map.get("fs")
            .expect("fs が存在しません")
            .get("file-max")
            .expect("file-max が存在しません"),
        "2097152"
    );
}

/// 再帰的なディレクトリ読み込みのテスト
#[test]
fn test_parse_all_sysctl_files() {
    let content1 = "net.ipv4.tcp_syncookies = 1";
    let content2 = "fs.file-max = 2097152";

    // ファイルをセットアップ
    let _ = setup_test_file("dir1/test1.conf", content1);
    let _ = setup_test_file("dir1/subdir/test2.conf", content2);

    // 再帰的にディレクトリを探索してパースする
    let directories = ["test_data/dir1"];
    let result = parse_all_sysctl_files(&directories);

    assert!(result.is_ok());
}
