#[cfg(test)]
mod tests {
    use linux_conf_parser::config::Config;
    use linux_conf_parser::core::directory_parser::DirectoryParser;
    use linux_conf_parser::core::file_parser::parse_conf_to_map;
    use linux_conf_parser::core::schema::validate_against_schema;
    use linux_conf_parser::core::schema::LoadSchema;
    use linux_conf_parser::core::{ParseFiles, SchemaLoader};
    use rustc_hash::FxHashMap;
    use std::fs::{self, File};
    use std::io::{self, Error, Write};
    use std::path::{Path, PathBuf};

    /// テスト用の一時ディレクトリとファイルを作成する関数
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
        let file_path: &Path = Path::new("non_existent.conf");
        let result: Result<FxHashMap<String, String>, Error> = parse_conf_to_map(file_path);
        assert!(result.is_err());
        if let Err(e) = result {
            assert_eq!(e.kind(), std::io::ErrorKind::NotFound);
        }
    }

    /// 4096文字を超える値が含まれている場合のエラーテスト
    #[test]
    #[should_panic(expected = "値が4096文字を超えています")]
    fn test_value_too_long() {
        let long_value: String = "A".repeat(Config::MAX_VALUE_LENGTH + 1);
        let content: String = format!("long.key = {}", long_value);
        let file_path: PathBuf = setup_test_file("long_value.conf", &content);

        let _ = parse_conf_to_map(&file_path);
        cleanup_test_files();
    }

    /// 正常な設定ファイルを読み込むテスト
    #[test]
    fn test_valid_conf_file() {
        let content: &str = "net.ipv4.tcp_syncookies = 1\nfs.file-max = 2097152";
        let file_path: PathBuf = setup_test_file("valid.conf", content);

        let result: Result<FxHashMap<String, String>, Error> = parse_conf_to_map(&file_path);
        assert!(result.is_ok(), "設定ファイルのパースに失敗しました");

        let map: FxHashMap<String, String> = result.unwrap();

        // マップ全体を表示して、デバッグしやすくする
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
    fn test_parse_all_conf_files() -> Result<(), Box<dyn std::error::Error>> {
        let content1: &str = "net.ipv4.tcp_syncookies = 1";
        let content2: &str = "fs.file-max = 2097152";

        // ファイルをセットアップ
        setup_test_file("dir1/test1.conf", content1);
        setup_test_file("dir1/subdir/test2.conf", content2);

        // 再帰的にディレクトリを探索してパースする
        let directories: [&str; 1] = ["test_data/dir1"];

        // スキーマファイルを読み込む
        let schema_path: &Path = Path::new("schema.txt");
        let schema_loader = LoadSchema;
        let schema: FxHashMap<String, String> = schema_loader.load_schema(schema_path)?;

        let mut result_map: FxHashMap<String, String> = FxHashMap::default();
        let parser = DirectoryParser; // スキーマローダーのインスタンスを作成
        let result: Result<(), Error> =
            parser.parse_all_conf_files(&directories, &schema, &mut result_map);

        // パース結果をデバッグ表示
        println!("パース結果: {:?}", result_map);

        // パースが成功したことを確認
        assert!(result.is_ok(), ".confファイルのパースに失敗しました");

        // パース結果の検証
        assert_eq!(
            result_map.get("net.ipv4.tcp_syncookies"),
            Some(&"1".to_string())
        );
        assert_eq!(result_map.get("fs.file-max"), Some(&"2097152".to_string()));

        // テスト後のクリーンアップ
        cleanup_test_files();

        Ok(())
    }

    /// テスト用のスキーマファイルをセットアップするヘルパー関数
    fn setup_test_schema(file_name: &str, content: &str) -> PathBuf {
        let test_dir: PathBuf = PathBuf::from("test_data");
        let file_path: PathBuf = test_dir.join(file_name);

        if let Some(parent_dir) = file_path.parent() {
            fs::create_dir_all(parent_dir).unwrap();
        }

        if file_path.exists() {
            fs::remove_file(&file_path).unwrap();
        }

        let mut file: File = File::create(&file_path).unwrap();
        file.write_all(content.as_bytes()).unwrap();
        file_path
    }

    /// 正常なスキーマファイルの読み込みテスト
    #[test]
    fn test_load_valid_schema() {
        let schema_content: &str = r#"
        key1 -> string
        key2 -> int
        key3 -> bool
        key4 -> float
        "#;
        let schema_path: PathBuf = setup_test_schema("valid_schema.txt", schema_content);
        let schema_loader = LoadSchema;
        let result: io::Result<FxHashMap<String, String>> = schema_loader.load_schema(&schema_path);
        assert!(result.is_ok(), "スキーマファイルの読み込みに失敗しました");

        let schema = result.unwrap();
        assert_eq!(schema.get("key1").unwrap(), "string");
        assert_eq!(schema.get("key2").unwrap(), "int");
        assert_eq!(schema.get("key3").unwrap(), "bool");
        assert_eq!(schema.get("key4").unwrap(), "float");

        cleanup_test_files();
    }

    /// 不正な形式のスキーマファイルの読み込みテスト
    #[test]
    fn test_load_invalid_schema() {
        let schema_content: &str = r#"
        key1 -> string
        invalid_format_line
        key2 -> int
        key3 -> float
        "#;
        let schema_path: PathBuf = setup_test_schema("invalid_schema.txt", schema_content);
        let schema_loader = LoadSchema;
        let result: io::Result<FxHashMap<String, String>> = schema_loader.load_schema(&schema_path);

        // エラーメッセージが適切に表示され、結果がエラーになることを確認
        assert!(result.is_ok(), "不正な形式の行を無視しなければなりません");

        let schema: FxHashMap<String, String> = result.unwrap();
        assert_eq!(schema.get("key1").unwrap(), "string");
        assert_eq!(schema.get("key2").unwrap(), "int");
        assert_eq!(schema.get("key3").unwrap(), "float");

        cleanup_test_files();
    }

    /// 浮動小数点数を含む設定ファイルの検証テスト
    #[test]
    fn test_validate_against_valid_schema_with_float() {
        let mut config: FxHashMap<String, String> = FxHashMap::default();
        config.insert("key1".to_string(), "value".to_string()); // 正しい string
        config.insert("key2".to_string(), "42".to_string()); // 正しい int
        config.insert("key3".to_string(), "true".to_string()); // 正しい bool
        config.insert("key4".to_string(), "3.14".to_string()); // 正しい float

        let mut schema: FxHashMap<String, String> = FxHashMap::default();
        schema.insert("key1".to_string(), "string".to_string());
        schema.insert("key2".to_string(), "int".to_string());
        schema.insert("key3".to_string(), "bool".to_string());
        schema.insert("key4".to_string(), "float".to_string());

        let result: Result<(), String> = validate_against_schema(&config, &schema);
        assert!(result.is_ok(), "検証に成功する必要があります");
    }

    /// 不正な型（整数に浮動小数点や文字列、ブールに文字列）が含まれている場合の検証テスト
    #[test]
    fn test_validate_mixed_invalid_types() {
        let mut config: FxHashMap<String, String> = FxHashMap::default();

        // 全て不正な値にする
        config.insert("key1".to_string(), "3.14".to_string()); // 不正な string (float が入っている)
        config.insert("key2".to_string(), "value".to_string()); // 不正な int (string が入っている)
        config.insert("key3".to_string(), "3.14".to_string()); // 不正な int (float が入っている)
        config.insert("key4".to_string(), "123".to_string()); // 不正な bool (int が入っている)
        config.insert("key5".to_string(), "value".to_string()); // 不正な bool (string が入っている)
        config.insert("key6".to_string(), "true".to_string()); // 不正な float (bool が入っている)
        config.insert("key7".to_string(), "true".to_string()); // 不正な string (bool が入っている)

        let mut schema: FxHashMap<String, String> = FxHashMap::default();

        schema.insert("key1".to_string(), "string".to_string()); // key1 は文字列でなければならない
        schema.insert("key2".to_string(), "int".to_string()); // key2 は整数でなければならない
        schema.insert("key3".to_string(), "int".to_string()); // key3 は整数でなければならない
        schema.insert("key4".to_string(), "bool".to_string()); // key4 はブール値でなければならない
        schema.insert("key5".to_string(), "bool".to_string()); // key5 はブール値でなければならない
        schema.insert("key6".to_string(), "float".to_string()); // key6 は浮動小数点でなければならない
        schema.insert("key7".to_string(), "string".to_string()); // key7 は文字列でなければならない

        let result: Result<(), String> = validate_against_schema(&config, &schema);

        assert!(result.is_err(), "検証は失敗する必要があります");

        let errors: String = result.unwrap_err();

        assert!(errors.contains(
            "Error: キー 'key1' の値 '3.14' の型が一致しません。期待される型は 'string'"
        ));
        assert!(errors
            .contains("Error: キー 'key2' の値 'value' の型が一致しません。期待される型は 'int'"));
        assert!(errors
            .contains("Error: キー 'key3' の値 '3.14' の型が一致しません。期待される型は 'int'"));
        assert!(errors
            .contains("Error: キー 'key4' の値 '123' の型が一致しません。期待される型は 'bool'"));
        assert!(errors
            .contains("Error: キー 'key5' の値 'value' の型が一致しません。期待される型は 'bool'"));
        assert!(errors
            .contains("Error: キー 'key6' の値 'true' の型が一致しません。期待される型は 'float'"));
        assert!(errors.contains(
            "Error: キー 'key7' の値 'true' の型が一致しません。期待される型は 'string'"
        ));
    }

    /// スキーマに存在しないキーを含む設定ファイルの検証テスト
    #[test]
    fn test_validate_with_extra_key() {
        let mut config: FxHashMap<String, String> = FxHashMap::default();
        config.insert("key1".to_string(), "value".to_string());
        config.insert("extra_key".to_string(), "value".to_string()); // スキーマに存在しないキー

        let mut schema: FxHashMap<String, String> = FxHashMap::default();
        schema.insert("key1".to_string(), "string".to_string());

        let result: Result<(), String> = validate_against_schema(&config, &schema);
        assert!(result.is_err(), "検証は失敗する必要があります");

        let errors: String = result.unwrap_err();
        assert!(errors.contains("キー 'extra_key' はスキーマに存在しません"));
    }
}
