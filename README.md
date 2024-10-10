# Linux sysctl.confパーサ (設定ファイルパーサ)
![CleanShot 2024-10-10 at 15 10 52](https://github.com/user-attachments/assets/6b6ebb4e-9727-4b88-baff-0e0f5db71239)

このRustプログラムは、`sysctl.conf`形式の設定ファイルを解析し、ネストされたキーと値のペアを`FxHashMap`に格納するパーサです。指定されたディレクトリを再帰的に探索し、`.conf`ファイルを読み込んで解析します。コメント行や空行を無視し、特定のエラーハンドリングにも対応しています。

## 機能

- **キーと値のペアを解析**: 設定ファイル内の`key=value`形式の行を解析し、`FxHashMap`に格納します。
- **ネストされたキーに対応**: `key.subkey=value`のようにドットで区切られたキーを、ネストされた`FxHashMap`として保存します。
- **コメント行や空行を無視**: `#`や`;`で始まるコメント行や空行は無視されます。
- **再帰的にディレクトリ内の`.conf`ファイルを解析**: 指定されたディレクトリ内の`.conf`ファイルを再帰的に読み込みます。
- **エラーハンドリング**: 行の先頭に`-`がある場合、その行で発生したエラーを無視し、それ以外のエラーは`stderr`に出力されます。

## ディレクトリ構成の準備

このプログラムを実行する前に、以下のようなディレクトリ構造を作成し、`.conf`ファイルを配置してください。

```
config/
├── etc/
│   ├── sysctl.conf
│   └── sysctl.d/
├── lib/
│   └── sysctl.d/
├── run/
│   └── sysctl.d/
├── usr/
│   ├── lib/
│   │   └── sysctl.d/
│   └── local/
│       └── lib/
│           └── sysctl.d/
```

各ディレクトリに`.conf`ファイルを配置して、システム設定を記述できます。

## 使用方法

### 1. 設定ファイルのフォーマット

設定ファイルは次の形式で記述します：

```bash
# コメント行
key1=value1
key2.subkey=value2
key3.subkey1.subkey2=value3

; こちらもコメント行
```

- `#`や`;`で始まる行はコメントとして無視されます。
- `key=value`形式の行はキーと値として解析されます。ドット（`.`）で区切られたキーはネストされたマップとして格納されます。
- 行の先頭に`-`が付いている場合、設定の適用エラーが発生してもエラーが無視されます。

### 2. プログラムの実行

以下のコマンドでプログラムを実行します。

```bash
cargo run
```

実行すると、指定された`config`ディレクトリ内のすべての`.conf`ファイルが再帰的に処理され、それぞれのファイルごとに以下のような形式で出力されます。

```
File: config/etc/sysctl.d/99-example.conf
fs
  file-max 2097152

net
  core
    somaxconn 1024
```

### 3. ディレクトリの指定

`main`関数では、以下のディレクトリリストが定義されています。このプログラムは、これらのディレクトリ内にある`.conf`ファイルを再帰的に読み込みます。

```rust
let directories = [
    "config/etc/sysctl.d",
    "config/run/sysctl.d",
    "config/usr/local/lib/sysctl.d",
    "config/usr/lib/sysctl.d",
    "config/lib/sysctl.d",
    "config/etc",
    "config"
];
```

このリストを変更することで、読み込みたいディレクトリを追加・削除できます。

### 4. カスタムエラーハンドリング

- 行の先頭に`-`が付いている場合、その行のエラーは無視されます。
- 設定値が4096文字を超える場合、警告が表示され、その行は無視されます。

## 4096文字を超えるファイル読み込みテスト
![CleanShot 2024-10-10 at 17 05 45](https://github.com/user-attachments/assets/8a47572a-1c58-4f44-9f87-a232bbcc9ee0)


このテストでは、`value.too.long` に4096文字を超える値が含まれる設定ファイルを読み込んだ際に、プログラムが正しくエラーメッセージを出力して終了することを確認します。

### 使用方法

#### 1. シェルスクリプトの実行

まず、以下のコマンドを実行して、`4096文字を超える設定ファイル` を自動生成します。

```sh
sh sh.sh
```

このスクリプトを実行すると、`config` ディレクトリ内に `long_value_test.conf` というファイルが作成されます。このファイルには、`value.too.long` キーに対して4096文字を超える値が含まれています。

#### 2. プログラムの実行

次に、以下のコマンドでプログラムを実行します。

```sh
cargo run
```

プログラムが `long_value_test.conf` を読み込むと、4096文字を超える値が検出されるため、エラーメッセージが表示されます。出力は以下のようになります。

```sh
File: "config/long_value_test.conf"
thread 'main' panicked at src/main.rs:35:17:
Error: キー 'value.too.long' の値が4096文字を超えています。👀
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

プログラムはこのエラーを検知すると、適切に終了します。これにより、長すぎる値が設定ファイルに含まれている場合、プログラムが異常な挙動を防ぎ、適切にエラーメッセージを出力して終了することを確認できます。



## 関数の説明

#### `parse_sysctl_conf(file_path: &Path) -> io::Result<FxHashMap<String, FxHashMap<String, String>>>`

指定されたファイルを読み込み、各行を解析して`FxHashMap`に格納します。行の先頭に`-`がある場合、その行のエラーは無視されます。

#### `insert_nested_key(map: &mut FxHashMap<String, FxHashMap<String, String>>, key: &str, value: &str)`

`key.subkey=value`のようにドットで区切られたキーを、ネストされたマップに変換して挿入します。

#### `parse_all_sysctl_files(directories: &[&str]) -> io::Result<()>`

複数のディレクトリを再帰的に探索し、すべての`.conf`ファイルを解析して出力します。

## 使用例

### 入力例 1

`config/example1.conf`ファイル：

```bash
endpoint = localhost:3000
debug = true
log.file = /var/log/console.log
```

### 出力例 1

```bash
File: config/example1.conf

endpoint localhost:3000
debug true
log
  file /var/log/console.log
```

### 入力例 2

`config/example2.conf`ファイル：

```bash
endpoint = localhost:3000
# debug = true
log.file = /var/log/console.log
log.name = default.log
```

### 出力例 2

```bash
File: config/example2.conf

endpoint localhost:3000
log
  file /var/log/console.log
  name default.log
```



## テスト仕様と使い方
![CleanShot 2024-10-10 at 22 38 57](https://github.com/user-attachments/assets/34e50125-253a-4674-84f1-18268459fef9)

このプログラムには、複数のテストが用意されています。各テストでは、`sysctl.conf`形式の設定ファイルを解析し、特定のケースに対応した動作を確認しています。テストは、特定のエラーハンドリングやファイルの正しい解析が行われているかを確認するためのものです。

### テストの実行方法

1. **`cargo test` コマンドを使用**  
   テストは `cargo test` コマンドで実行します。テスト用に定義された関数が順次実行され、結果が表示されます。

   ```bash
   cargo test
   ```

2. **各テストの動作**  
   テスト関数は、設定ファイルの内容に応じて、エラー処理や正常処理をテストします。以下で、各テストケースの概要を説明します。


### 1. `test_non_existent_file`

- **概要**: 存在しないファイルを開こうとした場合のエラーハンドリングを確認します。
- **期待結果**: 存在しないファイルにアクセスすると、`std::io::ErrorKind::NotFound` エラーが返されることを確認します。

```rust
#[test]
fn test_non_existent_file() {
    let file_path = Path::new("non_existent.conf");
    let result = parse_sysctl_conf(file_path);
    assert!(result.is_err());
    if let Err(e) = result {
        assert_eq!(e.kind(), std::io::ErrorKind::NotFound);
    }
}
```



### 2. `test_value_too_long`

- **概要**: 設定ファイルの値が4096文字を超えた場合に、パニックが発生することを確認します。
- **期待結果**: `should_panic` 属性によって、値が長すぎる際にプログラムがパニックすることを確認します。

```rust
#[test]
#[should_panic(expected = "値が4096文字を超えています")]
fn test_value_too_long() {
    let long_value = "A".repeat(MAX_VALUE_LENGTH + 1);
    let content = format!("long.key = {}", long_value);
    let file_path = setup_test_file("long_value.conf", &content);

    let result = parse_sysctl_conf(&file_path);
    assert!(result.is_err());

    if let Err(e) = result {
        assert_eq!(e.kind(), std::io::ErrorKind::InvalidData);
    }
}
```



### 3. `test_valid_conf_file`

- **概要**: 正常な設定ファイルを読み込み、内容が正しくパースされているかを確認します。
- **期待結果**: 設定ファイル内のキーと値が正しく `FxHashMap` に格納されていることを確認します。

```rust
#[test]
fn test_valid_conf_file() {
    let content = "net.ipv4.tcp_syncookies = 1\nfs.file-max = 2097152";
    let file_path = setup_test_file("valid.conf", content);

    let result = parse_sysctl_conf(&file_path);
    assert!(result.is_ok(), "設定ファイルのパースに失敗しました");

    let map = result.unwrap();
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
```



### 4. `test_parse_all_sysctl_files`

- **概要**: 複数のファイルがあるディレクトリを再帰的に読み込み、すべての設定ファイルを正しく解析できるかを確認します。
- **期待結果**: 再帰的なファイルの読み込みが成功し、すべてのキーと値が正しくパースされることを確認します。

```rust
#[test]
fn test_parse_all_sysctl_files() {
    let content1 = "net.ipv4.tcp_syncookies = 1";
    let content2 = "fs.file-max = 2097152";

    let _ = setup_test_file("dir1/test1.conf", content1);
    let _ = setup_test_file("dir1/subdir/test2.conf", content2);

    let directories = ["test_data/dir1"];
    let result = parse_all_sysctl_files(&directories);

    assert!(result.is_ok());

    // テスト後にクリーンアップ
    cleanup_test_files();
}
```



## テスト関数の使い方

- 各テスト関数は特定のシナリオに対して正しく動作するかを検証します。
- テスト実行後に一時的に作成されたファイルやディレクトリは、`cleanup_test_files` 関数を呼び出してクリーンアップします。

## テスト結果の確認方法

テストを実行すると、各テストケースが順番に実行されます。テストが成功すると "ok" が表示され、失敗するとエラーメッセージが表示されます。例えば、以下のような出力が得られます：

```bash
running 4 tests
test test_non_existent_file ... ok
test test_valid_conf_file ... ok
test test_value_too_long ... ok
test test_parse_all_sysctl_files ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

各テストが成功すれば問題なく動作しています。






# Linux sysctl.confパーサ (Map形式, JSON形式出力Ver.)
![CleanShot 2024-10-09 at 22 00 02](https://github.com/user-attachments/assets/2f3e0561-4975-41bc-8627-38f2c1e19408)

このRustプログラムは、`sysctl.conf` 形式に準拠した設定ファイルを解析し、ネストされたキーと値のペアを`serde_json::Map`に格納してMap形式, JSON形式で出力するパーサです。

リポジトリ(serde-map-jsonブランチ)↓
\
https://github.com/eternaleight/rust-projects/tree/serde-map-json

## 例

### 入力例 1

config/example1.conf

```bash
endpoint = localhost:3000
debug = true
log.file = /var/log/console.log
```

### 出力例 1

この設定ファイルを読み込むと、次のようにMap形式, JSON形式で出力されます

```bash
File: config/example1.conf

{"debug": String("true"), "endpoint": String("localhost:3000"), "log": Object {"file": String("/var/log/console.log")}}
Map

{
  "debug": "true",
  "endpoint": "localhost:3000",
  "log": {
    "file": "/var/log/console.log"
  }
}
JSON

```

### 入力例 2

config/example2.conf

```bash
endpoint = localhost:3000
# debug = true
log.file = /var/log/console.log
log.name = default.log
```

### 出力例 2

この設定ファイルを読み込むと、以下のように出力されます。`-` が付いた `kernel.hostname` は、エラーが発生しても無視されます。

```bash
File: config/example2.conf

{"endpoint": String("localhost:3000"), "log": Object {"file": String("/var/log/console.log"), "name": String("default.log")}}
Map

{
  "endpoint": "localhost:3000",
  "log": {
    "file": "/var/log/console.log",
    "name": "default.log"
  }
}
JSON
```
