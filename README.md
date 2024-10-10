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
