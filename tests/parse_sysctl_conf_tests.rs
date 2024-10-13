use projects::directory_parser::parse_all_sysctl_files;
use projects::file_parser::{parse_sysctl_conf, MAX_VALUE_LENGTH};
use projects::schema;
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

    // 作成されたファイルパスを表示（デバッグ用）
    println!("ファイル作成: {:?}", file_path);
    assert!(
        file_path.exists(),
        "ファイルが作成されていません: {:?}",
        file_path
    );

    file_path
}

fn cleanup_test_files() {
    if fs::remove_dir_all("test_data").is_err() {
        println!("テストデータのクリーンアップに失敗しました");
    } else {
        println!("テストデータのクリーンアップが成功しました");
    }
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

    let _ = parse_sysctl_conf(&file_path);
    cleanup_test_files();
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
        map.get("net.ipv4.tcp_syncookies")
            .expect("tcp_syncookies が存在しません"),
        "1"
    );
    assert_eq!(
        map.get("fs.file-max").expect("file-max が存在しません"),
        "2097152"
    );
    cleanup_test_files();
}

/// 再帰的なディレクトリ読み込みのテスト
#[test]
fn test_parse_all_sysctl_files() -> Result<(), Box<dyn std::error::Error>> {
    let content1 = "net.ipv4.tcp_syncookies = 1";
    let content2 = "fs.file-max = 2097152";

    // ファイルをセットアップ
    let _ = setup_test_file("dir1/test1.conf", content1);
    let _ = setup_test_file("dir1/subdir/test2.conf", content2);

    // 再帰的にディレクトリを探索してパースする
    let directories = ["test_data/dir1"];

    // スキーマファイルを読み込む
    let schema_path = Path::new("schema.txt");
    let schema = schema::load_schema(schema_path)?;

    let result = parse_all_sysctl_files(&directories, &schema);

    // パース結果をデバッグ表示
    println!("パース結果: {:?}", result);

    // パースが成功したことを確認
    assert!(result.is_ok(), "Sysctlファイルのパースに失敗しました");

    if let Ok(map) = result {
        // 期待するキーと値が存在するか確認
        println!("map: {:?}", map); // マップ全体を表示してデバッグ
        assert_eq!(
            map.get("net.ipv4.tcp_syncookies")
                .expect("net.ipv4.tcp_syncookiesの値が期待と異なります"),
            "1"
        );
        assert_eq!(
            map.get("fs.file-max")
                .expect("fs.file-maxの値が期待と異なります"),
            "2097152"
        );
    }
    cleanup_test_files();

    Ok(())
}
