# Linux sysctl.confパーサ (設定ファイルパーサ)
🔨作成中(WIP)

![CleanShot 2024-10-14 at 02 44 47](https://github.com/user-attachments/assets/d05ac8b0-37eb-42cd-b605-51919885eacd)
![CleanShot 2024-10-14 at 02 46 22](https://github.com/user-attachments/assets/aa526b6c-cadb-4640-bc94-3951bfaec1fd)

このRustプログラムは、`sysctl.conf`形式の設定ファイルを解析し、ネストされたキーと値のペアを`FxHashMap`に格納するパーサです。指定されたディレクトリを再帰的に探索し、`.conf`ファイルを読み込んで解析します。コメント行や空行を無視し、特定のエラーハンドリングにも対応しています。

## 機能

- **キーと値のペアを解析**: 設定ファイル内の`key=value`形式の行を解析し、`FxHashMap`に格納します。
- **ネストされたキーに対応**: `key.subkey=value`のようにドットで区切られたキーを、ネストされた`FxHashMap`として保存します。
- **コメント行や空行を無視**: `#`や`;`で始まるコメント行や空行は無視されます。
- **再帰的にディレクトリ内の`.conf`ファイルを解析**: 指定されたディレクトリ内の`.conf`ファイルを再帰的に読み込みます。
- **エラーハンドリング**: 行の先頭に`-`がある場合、その行で発生したエラーを無視し、それ以外のエラーは`stderr`に出力されます。


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
File: config/etc/sysctl.d/99-example.conf
fs
  file-max 2097152

net
  core
    somaxconn 1024
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
    "/etc"
];
```

### 本番システムでの使用方法

本番システムでこのプログラムを使用する場合、以下の手順に従ってください。

1. **開発用コードをコメントアウトし、本番システム用のコードを有効にする**  
   現在、ソースコード内に開発用と本番用のコードが共存しています。実際に本番システムで動作させる際は、開発用のコードをコメントアウトし、以下の部分を有効化してください。

   ```rust
   // 開発用コードをコメントアウト
   // mod parser;
   // mod utils;
   ...
   
   // 本番システム用コードを有効化
   mod parser;
   mod utils;

   use std::io;

   fn main() -> io::Result<()> {
   // 再帰的に探索するディレクトリ
       let directories = [
           "/etc/sysctl.d",
           "/run/sysctl.d",
           "/usr/local/lib/sysctl.d",
           "/usr/lib/sysctl.d",
           "/lib/sysctl.d",
           "/etc",
       ];

       parser::parse_all_sysctl_files(&directories)?;

       Ok(())
   }
   ```

2. **本番システムでの動作確認**  
   上記の変更を行った後、本番システムに適したディレクトリ構造と設定ファイルを確認し、プログラムを実行してください。

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

- **概要**: 設定ファイルの値が4096文字を超えた場合に、プログラムがパニックを発生させることを確認します。このテストは、システムの安全性を保持するために、特定の長さを超える設定値に対して厳格な制限を施すことの重要性を強調します。
- **期待結果**: `should_panic` 属性を使用して、設定値が4096文字を超えるときにプログラムがパニックすることを確認します。このパニックは、設定値が長すぎるために予期せず起こる状態を模倣し、適切なエラー処理とシステムの安全性を保証します。

```rust
#[test]
#[should_panic(expected = "値が4096文字を超えています")]
fn test_value_too_long() {
    let long_value = "A".repeat(MAX_VALUE_LENGTH + 1);
    let content = format!("long.key = {}", long_value);
    let file_path = setup_test_file("long_value.conf", &content);

    // この関数呼び出しは panic を引き起こすことが期待されている
    let _ = parse_sysctl_conf(&file_path);
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
🔨作成中(WIP)

```rust
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

    cleanup_test_files();

    Ok(())
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

