# Linux sysctl.confパーサ (設定ファイルパーサ)
🔨作成中(WIP)

| ![CleanShot 2024-10-14 at 02 44 47](https://github.com/user-attachments/assets/d05ac8b0-37eb-42cd-b605-51919885eacd) |
|:--:|
| cargo runコマンド（設定ファイルをJSON形式で出力し、パース結果、型の検証結果を表示）|

| ![CleanShot 2024-10-17 at 15 54 31](https://github.com/user-attachments/assets/c9bac18e-0e7b-4917-b129-c9bdad16f7f5) |
|:--:|
| cargo run outputコマンド（設定ファイルを元に、空の型定義ファイル `output.txt` を作成）|

| ![CleanShot 2024-10-17 at 15 57 47](https://github.com/user-attachments/assets/135e5a60-1027-4560-9b5d-4cd2fb622a75) |
|:--:|
|linux-conf-parserコマンド (cargo runと機能は同じ、バイナリで実行)|

### このRustプログラムについて

このRustプログラムは、`sysctl.conf`形式の設定ファイルを解析し、ネストされたキーと値のペアを`FxHashMap`に格納するパーサです。指定されたディレクトリを再帰的に探索し、`.conf`ファイルを読み込んで解析します。コメント行や空行を無視し、特定のエラーハンドリングにも対応しています。 
型定義ファイルを作成し、型定義を行うとそれに基づき設定ファイルのキーと値の型を検証することが可能です。設定ファイルの内容が期待されるデータ型と一致しているかを確認し、不正な値が含まれている場合にはエラーメッセージを表示します。

## 機能

- **キーと値のペアを解析**: 設定ファイル内の`key=value`形式の行を解析し、`FxHashMap`に格納します。
- **ネストされたキーに対応**: `key.subkey=value`のようにドットで区切られたキーを、ネストされた`FxHashMap`として保存します。
- **コメント行や空行を無視**: `#`や`;`で始まるコメント行や空行は無視されます。
- **再帰的にディレクトリ内の`.conf`ファイルを解析**: 指定されたディレクトリ内の`.conf`ファイルを再帰的に読み込みます。
- **エラーハンドリング**: 行の先頭に`-`がある場合、その行で発生したエラーを無視し、それ以外のエラーは`stderr`に出力されます。
- **型定義ファイルの作成と検証**: 空の型定義ファイルを生成し、設定ファイルの各キーと値の型を定義します（各設定項目のstring型 String, int型 i64, bool型 bool, float型 f64を指定します）。定義された型に基づいて、設定ファイルの内容が正しいかどうかをチェック。
```bash
型定義ファイルの作成例
例：schema.txt

log.file -> string
net.ipv4.tcp_syncookies -> int
debug -> bool
net.ipv4.tcp_rmem -> float
```


### 開発用ディレクトリとファイルの説明

このプログラムは、開発用と本番システム用で異なるディレクトリ構成を使用します。開発環境では、あらかじめ用意された仮想的なディレクトリ構造を使用して動作します。以下は、開発用に仮想的に設定されているディレクトリとファイル構成の説明です。

### 開発環境でのディレクトリ構成

開発用には、`config/`以下に以下のディレクトリ構造が設定されています。この構造内で`.conf`ファイルを読み込み、システム設定を模擬的に処理します。

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

この仮想的なディレクトリ構成をもとに、各ディレクトリに`.conf`ファイルが存在する想定で動作し、再帰的にそれらのファイルを読み込んで設定を処理します。

### 本番システムでのディレクトリ構成

本番システム用では、実際のシステムディレクトリである`/etc/`や`/usr/lib/`などが使用されます。仮想ディレクトリではなく、システム上の実際のディレクトリを対象とするため、ファイルパスやアクセス権などの設定にも注意が必要です。

```
/
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

本番システム用のコードは、ルートディレクトリにある実際のシステムファイルを処理するように記述されています。


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
File: "config/example1.conf"
{
  "debug": "true",
  "endpoint": "localhost:3000",
  "log": {
    "file": "/var/log/console.log"
  }
}
```

#### 2.2 `linux-conf-parser` コマンドでの実行（バイナリで実行、処理速度は速い、コード変更時にコンパイル必要）

バイナリとしてインストールした `linux-conf-parser` コマンドを使用して、同様の処理を実行できます。バイナリは、事前に次のコマンドを使用してグローバルにインストールする必要があります。

このコマンドは、自動的にビルドプロセス(` cargo build --release `)も含まれるため、別途 `cargo build --release` を実行する必要はありません。
```bash
cargo install --path .
```

インストールが完了したら、以下のコマンドでプログラムを実行します。

```bash
linux-conf-parser
```

コード変更などをした場合は、再度このコマンドを入力してコンパイル。
```bash
cargo install --path .
```

### 3. ディレクトリの指定

開発環境向けのコードでは、以下のディレクトリリストが定義されています。このプログラムは、これらのディレクトリ内にある`.conf`ファイルを再帰的に読み込みます。

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

### 本番システム用のディレクトリ

本番システムでの使用に向けて、以下のディレクトリリストを使用します。本番環境では、実際のシステムディレクトリを対象とするため、開発時とは異なるパスが指定されています。

```rust
let directories = [
    "/etc/sysctl.d",
    "/run/sysctl.d",
    "/usr/local/lib/sysctl.d",
    "/usr/lib/sysctl.d",
    "/lib/sysctl.d",
];
```

### 本番システムでの使用方法

本番システムでこのプログラムを使用する場合、以下の手順に従ってください。

1. **開発用コードをコメントアウトし、本番システム用のコードを有効にする**  
   現在、ソースコード内に開発用と本番用のコードが共存しています。実際に本番システムで動作させる際は、開発用のコードをコメントアウトし、以下の部分を本番システム用ディレクトリに入れ替えて使用してください。

   ```rust
   // main.rs
   
   // 開発用コードをコメントアウト
   // mod core;
   // mod utils;
   ...

   // 本番想定ディレクトリ
   fn main() -> io::Result<()> {
   // 再帰的に探索するディレクトリ
       let directories = [
           "/etc/sysctl.d",
           "/run/sysctl.d",
           "/usr/local/lib/sysctl.d",
           "/usr/lib/sysctl.d",
           "/lib/sysctl.d",
       ];
   ...
   
   }
   ```

2. **本番システムでの動作確認**  
   上記の変更を行った後、本番システムに適したディレクトリ構造と設定ファイルを確認し、プログラムを実行してください。

### 4. カスタムエラーハンドリング

- 行の先頭に`-`が付いている場合、その行のエラーは無視されます。
- 設定値が4096文字を超える場合、警告が表示され、その行は無視されます。

## 型定義ファイルの作成と検証
![CleanShot 2024-10-17 at 15 54 31](https://github.com/user-attachments/assets/61d39e9a-7ab0-489e-b298-e9830dee7bb0)

### `cargo run output` で空の型定義ファイルを作成

1. `cargo run output` を実行し、空の型定義ファイル `output.txt` を作成します。
    ```
    cargo run output
    ```

    このコマンドは、システムの設定に基づいて空の型定義ファイルを生成します。次の手順でこのファイルを型定義ファイルとして使用します。

3. `output.txt` を `schema.txt` に名前を変更します。このファイルは、型定義ファイルとして使用されます。schema.txtで型の定義を行って下さい。

    ```bash
    mv output.txt schema.txt
    ```

### `cargo run` で.confファイルをJSON形式で出力し、型の検証を行う

3. 型定義ファイル（`schema.txt`）を使って、システムの `.conf` ファイルの設定を検証し、JSON形式で出力します。以下のコマンドを実行してください。

    ```bash
    cargo run
    ```

- **動作**: このコマンドは、`.conf` ファイルを解析し、型定義ファイルに基づいて設定の正当性を検証します。
- **出力**: 設定ファイルの内容をJSON形式で表示し、型が不一致の場合はエラーメッセージが表示されます。
### シンプルな手順例
```bash
# 空の型定義ファイルを生成
cargo run output

# output.txtをschema.txtに名前変更を変更して、schema.txtで型の定義を行う
mv output.txt schema.txt

# .confファイルの設定をJSON 形式で出力し、型の検証結果も表示
cargo run

## 使用方法

型定義ファイルは次の形式で記述します

例：schema.txt

log.file -> string
endpoint -> string
net.ipv4.tcp_syncookies -> int
kernel.modprobe ->string
debug -> bool
kernel.sysrq -> int
log.name -> string
kernel.domainname -> string
net.ipv4.tcp_rmem -> int
kernel.panic -> string
vm.swappiness -> int
fs.file-max -> int
```

### 設定ファイル例と型の不一致
```bash
example1.conf
endpoint = localhost:3000
debug = 1234 ← bool型に文字列を入れている
log.file = /var/log/console.log

99-example.conf
kernel.sysrq = '`'`.|¥/;""?'` ← int型に文字列を入れている

20-extra.conf
net.ipv4.tcp_rmem = asdf ← int型に文字列を入れている

10-custom.conf
vm.swappiness = 10.1 ← int型に浮動小数点数を入れている
fs.file-max = 100000
```

以下のように表示されます
### エラーメッセージ例：
```bash
Error: キー 'debug' の値 '1234' の型が一致しません。期待される型は 'bool'
Error: キー 'kernel.sysrq' の値 '`'`.|¥/;""?`' の型が一致しません。期待される型は 'int'
Error: キー 'net.ipv4.tcp_rmem' の値 'asdf' の型が一致しません。期待される型は 'float'
Error: キー 'vm.swappiness' の値 '10.1' の型が一致しません。期待される型は 'int'
設定ファイルのパース中にエラーが発生しました: 設定ファイルにエラーがあります。
```

または、スキーマ(schema.txt)の定義でこのように定義されていない型を入力すると
```
例：schema.txt

log.file -> #空文字
endpoint -> asdf
vm.swappiness ->'string' #クォートで囲む
```

以下のように表示されます
```bash
Error: キー 'log.file' のスキーマ型 '' はサポートされていません。
Error: キー 'endpoint' のスキーマ型 'asdf' はサポートされていません。
Error: キー 'vm.swappiness' のスキーマ型 ''string'' はサポートされていません。
```


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
File: "config/example1.conf"
{
  "debug": "true",
  "endpoint": "localhost:3000",
  "log": {
    "file": "/var/log/console.log"
  }
}
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
File: "config/example2.conf"
{
  "endpoint": "localhost:3000",
  "log": {
    "file": "/var/log/console.log",
    "name": "default.log"
  }
}
```


## テスト仕様と使い方
![CleanShot 2024-10-16 at 20 37 50](https://github.com/user-attachments/assets/6d649b7a-9959-4875-8559-b8881a3e3e66)

このプログラムには、複数のテストが用意されています。各テストでは、`sysctl.conf`形式の設定ファイルを解析し、特定のケースに対応した動作を確認しています。テストは、特定のエラーハンドリングやファイルの正しい解析が行われているかを確認するためのものです。

### テストの実行方法

1. **`cargo test` コマンドを使用**  
   テストは `cargo test` コマンドで実行します。テスト用に定義された関数が順次実行され、結果が表示されます。

   ```bash
   # テストを逐次実行したいのでこのコマンドで実行する
   cargo test -- --test-threads=1
   ```

**`cargo test`**:
   - RustのビルドシステムであるCargoを使用して、プロジェクト内の全てのテストを実行するコマンドです。`cargo test`を実行すると、テスト対象の関数が実行され、結果が表示されます。

**`--`**:
   - `cargo test` コマンドに渡すオプションと、テストランナー（Rustのテスト実行エンジン）に渡すオプションを区別するための区切りです。これにより、後ろに続くオプションはテストランナーに渡されます。

**`--test-threads=1`**:
   - これはテストランナーに渡されるオプションで、テストを実行するスレッドの数を指定します。この例では、`1` つのスレッドを指定しているため、テストは並列に実行されず、順次実行されます。

Rustのテストランナーは複数のスレッドを使用して並列にテストを実行しますが、`--test-threads=1` を指定することで、全てのテストを1つのスレッドで順次実行します。
- テストが並列実行されると、データ競合やリソースの共有によって問題が発生する場合。
- テストの実行順序が重要な場合。
- 並行実行によってデバッグが難しくなる問題を回避したい場合。


2. **各テストの動作**  
   テスト関数は、設定ファイルの内容に応じて、エラー処理や正常処理をテストします。以下で、各テストケースの概要を説明します。


### 1. `test_non_existent_file`

- **概要**: 存在しないファイルを開こうとした場合のエラーハンドリングを確認します。
- **期待結果**: 存在しないファイルにアクセスすると、`std::io::ErrorKind::NotFound` エラーが返されることを確認します。

```rust
#[test]
fn test_non_existent_file() {
    let file_path: &Path = Path::new("non_existent.conf");
    let result: Result<FxHashMap<String, String>, Error> = parse_sysctl_conf(file_path);
    assert!(result.is_err());
    if let Err(e) = result {
        assert_eq!(e.kind(), std::io::ErrorKind::NotFound);
    }
}
```

### 2. `test_value_too_long`

- **概要**: 設定ファイルの値が4096文字を超えた場合に、プログラムがパニックを発生させることを確認します。このテストは、システムの安全性を保持するために、特定の長さを超える設定値に対して厳格な制限を施すことの重要性を強調します。
- **期待結果**: `should_panic` 属性を使用して、設定値が4096文字を超えるときにプログラムがパニックすることを確認します。このパニックは、設定値が長すぎるために予期せず起こる状態を模倣し、適切なエラー処理とシステムの安全性を保証します。

```rust
#[test]
#[should_panic(expected = "値が4096文字を超えています")]
fn test_value_too_long() {
    let long_value: String = "A".repeat(MAX_VALUE_LENGTH + 1);
    let content: String = format!("long.key = {}", long_value);
    let file_path: PathBuf = setup_test_file("long_value.conf", &content);

    // この関数呼び出しは panic を引き起こすことが期待されている
    let _ = parse_sysctl_conf(&file_path);
    cleanup_test_files();
}
```

### 3. `test_valid_conf_file`

- **概要**: 正常な設定ファイルを読み込み、内容が正しくパースされているかを確認します。
- **期待結果**: 設定ファイル内のキーと値が正しく `FxHashMap` に格納されていることを確認します。

```rust
#[test]
fn test_valid_conf_file() {
    let content: &str = "net.ipv4.tcp_syncookies = 1\nfs.file-max = 2097152";
    let file_path: PathBuf = setup_test_file("valid.conf", content);

    let result: Result<FxHashMap<String, String>, Error> = parse_sysctl_conf(&file_path);
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
```

### 4. `test_parse_all_sysctl_files`

- **概要**: 再帰的にディレクトリを探索し、`sysctl` 設定ファイルを正しく読み込んでパースするかを確認するテストです。複数のディレクトリ内に保存された設定ファイルを読み込み、それぞれの設定項目がスキーマに基づいて適切に処理されるかどうかを検証します。
- **期待結果**: 各 `sysctl` 設定ファイルが再帰的にディレクトリ内から正しく読み込まれ、パースされた結果が `FxHashMap` に期待通りに格納されること。また、スキーマに基づいた検証が成功し、エラーなく結果が返されること。

```rust
/// 再帰的なディレクトリ読み込みのテスト
#[test]
fn test_parse_all_sysctl_files() -> Result<(), Box<dyn std::error::Error>> {
    let content1: &str = "net.ipv4.tcp_syncookies = 1";
    let content2: &str = "fs.file-max = 2097152";

    // ファイルをセットアップ
    setup_test_file("dir1/test1.conf", content1);
    setup_test_file("dir1/subdir/test2.conf", content2);

    // 再帰的にディレクトリを探索してパースする
    let directories: [&str; 1] = ["test_data/dir1"];

    // スキーマファイルを読み込む
    let schema_path: &Path = Path::new("schema.txt");
    let schema: FxHashMap<String, String> = schema::load_schema(schema_path)?;

    let mut result_map: FxHashMap<String, String> = FxHashMap::default();
    let result: Result<(), Error> = parse_all_sysctl_files(&directories, &schema, &mut result_map);

    // パース結果をデバッグ表示
    println!("パース結果: {:?}", result_map);

    // パースが成功したことを確認
    assert!(result.is_ok(), "Sysctlファイルのパースに失敗しました");

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
```

### 5. `test_load_valid_schema`

- **概要**: 正常なスキーマファイルを読み込み、その内容が正しく解析されているかを確認するテストです。スキーマの各項目が期待通りのデータ型と対応しているかを検証します。
- **期待結果**: スキーマが正しくロードされ、各キーに対応する型（`string`、`int`、`bool`、`float`）が `FxHashMap` に格納されていることを確認します。

```rust
#[test]
fn test_load_valid_schema() {
    let schema_content: &str = r#"
    key1 -> string
    key2 -> int
    key3 -> bool
    key4 -> float
    "#;
    let schema_path: PathBuf = setup_test_schema("valid_schema.txt", schema_content);
    let result: Result<FxHashMap<String, String>, Error> = load_schema(&schema_path);
    assert!(result.is_ok(), "スキーマファイルの読み込みに失敗しました");

    let schema = result.unwrap();
    assert_eq!(schema.get("key1").unwrap(), "string");
    assert_eq!(schema.get("key2").unwrap(), "int");
    assert_eq!(schema.get("key3").unwrap(), "bool");
    assert_eq!(schema.get("key4").unwrap(), "float");

    cleanup_test_files();
}
```

### 6. `test_load_invalid_schema`

- **概要**: 不正な形式を含むスキーマファイルを読み込み、エラーハンドリングが適切に行われるかを確認するテストです。不正な行があっても、残りのスキーマ項目が正しくロードされるかを検証します。
- **期待結果**: スキーマファイル内の不正な行は無視され、他の正しい行が正しくパースされること。

```rust
#[test]
fn test_load_invalid_schema() {
    let schema_content: &str = r#"
    key1 -> string
    invalid_format_line
    key2 -> int
    key3 -> float
    "#;
    let schema_path: PathBuf = setup_test_schema("invalid_schema.txt", schema_content);
    let result: Result<FxHashMap<String, String>, Error> = load_schema(&schema_path);

    // エラーメッセージが適切に表示され、結果がエラーになることを確認
    assert!(result.is_ok(), "不正な形式の行を無視しなければなりません");

    let schema: FxHashMap<String, String> = result.unwrap();
    assert_eq!(schema.get("key1").unwrap(), "string");
    assert_eq!(schema.get("key2").unwrap(), "int");
    assert_eq!(schema.get("key3").unwrap(), "float");

    cleanup_test_files();
}
```

### 7. `test_validate_against_valid_schema_with_float`

- **概要**: 浮動小数点数を含む設定ファイルの内容が、スキーマに基づいて正しく検証されるかを確認するテストです。複数の異なるデータ型がスキーマに対して適切に処理されるかを検証します。
- **期待結果**: 各設定項目（`string`、`int`、`bool`、`float`）がスキーマに基づいて正しく検証され、エラーが発生しないこと。

```rust
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
```

### 8. `test_validate_with_extra_key`

- **概要**: スキーマには定義されていない余分なキーが設定ファイルに含まれている場合、そのキーが適切に検出されるかを確認するテストです。余分なキーがエラーとして扱われるかを検証します。
- **期待結果**: スキーマに存在しないキーが検出され、エラーメッセージが返されること。

```rust
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
```

### 9. `test_validate_mixed_invalid_types`

- **概要**: 無効なデータ型が複数含まれている設定ファイルが、スキーマに基づいて正しくエラーハンドリングされるかを確認するテストです。各項目がスキーマに準拠していない場合、適切なエラーメッセージが返されるかを検証します。
- **期待結果**: 無効なデータ型に対して適切なエラーメッセージが表示され、検証が失敗すること。

```rust
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

    assert!(errors.contains("Error: キー 'key1' の値 '3.14' の型が一致しません。期待される型は 'string'"));
    assert!(errors.contains("Error: キー 'key2' の値 'value' の型が一致しません。期待される型は 'int'"));
    assert!(errors.contains("Error: キー 'key3' の値 '3.14' の型が一致しません。期待される型は 'int'"));
    assert!(errors.contains("Error: キー 'key4' の値 '123' の型が一致しません。期待される型は 'bool'"));
    assert!(errors.contains("Error: キー 'key5' の値 'value' の型が一致しません。期待される型は 'bool'"));
    assert!(errors.contains("Error: キー 'key6' の値 'true' の型が一致しません。期待される型は 'float'"));
    assert!(errors.contains("Error: キー 'key7' の値 'true' の型が一致しません。期待される型は 'string'"));
}
```

## テスト関数の使い方

- 各テスト関数は特定のシナリオに対して正しく動作するかを検証します。
- テスト実行後に一時的に作成されたファイルやディレクトリは、`cleanup_test_files` 関数を呼び出してクリーンアップします。

## テスト結果の確認方法

テストを実行すると、各テストケースが順番に実行されます。テストが成功すると "ok" が表示され、失敗するとエラーメッセージが表示されます。例えば、以下のような出力が得られます。

### 実行例

```bash
running 9 tests
test test_non_existent_file ... ok
test test_parse_all_sysctl_files ... ok
test test_valid_conf_file ... ok
test test_value_too_long - should panic ... ok
test tests::test_load_invalid_schema ... ok
test tests::test_load_valid_schema ... ok
test tests::test_validate_against_valid_schema_with_float ... ok
test tests::test_validate_mixed_invalid_types ... ok
test tests::test_validate_with_extra_key ... ok

test result: ok. 9 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s
```

各テストが成功すれば問題なく動作しています。


## ベンチマークテスト
![CleanShot 2024-10-11 at 16 51 30](https://github.com/user-attachments/assets/fa2d54dc-92c0-4c66-9ba6-00f49547951d)


以下の手順に従って、Rustプロジェクト内でベンチマークテストを実行できます。

## 1. **nightly ツールチェーンのインストール**

ベンチマークを実行するには、Rustの `nightly` ツールチェーンが必要です。以下のコマンドでインストールしてください。

```bash
rustup install nightly
```


## 2. **ベンチマークの実行**

次のコマンドを使って、ベンチマークテストを実行します。
```bash
cargo +nightly bench
```
`stableチャンネル` Rustのデフォルトチャンネルのまま実行できる。
または
```bash
cargo bench
```
`nightly` チャンネルで実行できる。


`stable` チャンネルでは、**安定版の機能のみ**が使用でき、バグ修正やセキュリティアップデートが含まれています。新機能は、まず `nightly` チャンネルでテストされ、その後 `beta` を経て `stable` に導入されます。

### 補足情報:
- **`stable` チャンネル**は、Rustの最も安定したリリースです。これはプロダクション用途での使用が推奨されており、予期しない破壊的な変更が行われることはありません。
- **`nightly` チャンネル**は、実験的な新機能や最先端の変更が含まれるため、開発中の新機能を試したり、ベンチマークのような一部の特定機能を使用したりする際に必要です。


## nightly チャンネルに切り替える

### 1. **nightly チャンネルの設定**

プロジェクトディレクトリ内で `nightly` ツールチェーンを使用するように設定します。

```bash
rustup override set nightly
```

これで、このプロジェクトでは `nightly` がデフォルトで使用されます。

### 2. インストールの確認
インストールが完了したら、以下のコマンドを使用してnightlyツールチェーンが正しくインストールされたことを確認できます。
```bash
rustup show
```

### 3. **stable チャンネルに戻す**

プロジェクトディレクトリ内で `stable` ツールチェーンを使用するように設定します。

```bash
rustup override set stable
```

stable チャンネルに戻ります。


## 3. **結果の確認**

実行後、各ベンチマークの実行時間がナノ秒単位で表示されます。以下のような結果が表示されます。

```
test benchmarks::bench_empty_conf_file        ... bench:     114,022.41 ns/iter (+/- 48,714.07)
test benchmarks::bench_large_conf_file        ... bench:     636,778.90 ns/iter (+/- 247,165.45)
test benchmarks::bench_parse_all_sysctl_files ... bench:     498,996.82 ns/iter (+/- 311,588.69)
test benchmarks::bench_parse_sysctl_conf      ... bench:     377,210.36 ns/iter (+/- 33,725.80)
```

- **ns/iter**: 1回の処理にかかった時間（ナノ秒）。
- **(+/- XXX)**: 標準偏差。処理時間のばらつきを示します。

