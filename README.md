# Linux sysctl.conf パーサ (設定ファイルパーサ)

このRustプログラムは、`sysctl.conf` 形式に準拠した設定ファイルを解析し、ネストされたキーと値のペアを辞書型に格納してJSON形式で出力するパーサです。

## 機能

- **キーと値のペアを解析**: 設定ファイル内の `key=value` 形式の行をパースして、設定を辞書型に格納します。
- **ネストされたキーに対応**: ドット (`.`) で区切られたキー (`key.subkey.subsubkey=value`) をネストされたJSONオブジェクトに変換します。
- **コメントや空行を無視**: `#` や `;` で始まるコメント行、または空行を無視します。
- **ファイルごとにJSONを出力**: 指定したディレクトリ内のすべての `.conf` ファイルを再帰的に読み込み、各ファイルごとにJSON形式でその内容を出力します。
- **エラー処理**: 行の先頭に `-` がある場合、設定の適用エラーが発生してもそのエラーを無視して処理を継続します。それ以外のエラーは標準エラー出力 (`stderr`) に出力されます。

## ディレクトリ構成の準備

本プログラムを実行するには、以下のようなディレクトリ構造を作成し、`.conf`ファイルを配置してください：

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

各ディレクトリには `.conf` ファイルを配置し、システム設定を記述することができます。

## 使用方法

### 1. 設定ファイルのフォーマット

設定ファイルは以下の形式で記述されます：

```bash
# コメント行
key1=value1
key2.subkey=value2
key3.subkey1.subkey2=value3

; こちらもコメント行
```

- `#` や `;` で始まる行はコメントとして無視されます。
- `key=value` の形式で記述された行はキーと値として認識されます。ドット (`.`) で区切られたキーは、ネストされたJSONオブジェクトに変換されます。
- 行の先頭に `-` が付いている場合、設定の適用に失敗してもエラーが無視されます。

### 2. プログラムの実行

このプログラムは指定されたディレクトリ内の設定ファイルを再帰的に読み込み、各ファイルごとにその内容をJSON形式で標準出力に出力します。以下のコマンドで実行します：

```bash
cargo run
```

実行すると、指定した `config` ディレクトリに存在するすべての`.conf`ファイルを処理し、各ファイルごとに以下のような形式で出力します：

```
File: config/etc/sysctl.d/99-example.conf
{
  "fs": {
    "file-max": "2097152"
  },
  "net": {
    "core": {
      "somaxconn": "1024"
    }
  }
}

File: config/run/sysctl.d/99-example.conf
{
  "net": {
    "ipv4": {
      "tcp_syncookies": "1"
    },
    "ipv4.conf.all": {
      "rp_filter": "1"
    }
  }
}
```

### 3. ディレクトリの指定

`main` 関数では、以下のディレクトリリストが定義されています。プログラムはこれらのディレクトリ内にある `.conf` ファイルを再帰的に読み込みます。

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

このリストを変更することで、読み込みたいディレクトリを追加・削除することができます。

### 4. カスタマイズされたエラーハンドリング

- `-` で始まる行は、適用エラーが無視されます。
- 設定値の長さが4096文字を超える場合、警告が出力され、処理が停止します。

## 関数の説明

### `parse_sysctl_conf(file_path: &Path) -> io::Result<Map<String, Value>>`

- 指定されたファイルを読み込み、各行を解析して辞書型に格納します。行の先頭に `-` がある場合、その行で発生したエラーは無視されます。

### `insert_nested_key(map: &mut Map<String, Value>, key: &str, value: &str)`

- ドットで区切られたキー（例：`key.subkey.subsubkey=value`）をネストされたJSONオブジェクト形式に変換して `map` に挿入します。

### `parse_all_sysctl_files(directories: &[&str]) -> io::Result<()>`

- 複数のディレクトリを再帰的に探索し、すべての `.conf` ファイルを解析して内容をJSON形式で出力します。

## 例

### 入力例 1

`以下の設定ファイル `config/sysctl.conf`

```bash
# システム設定
kernel.hostname=myserver
kernel.max_files=10000
net.ipv4.ip_forward=1
```

### 出力例 1

この設定ファイルを読み込むと、次のようにJSON形式で出力されます

```json
{
  "kernel": {
    "hostname": "myserver",
    "max_files": "10000"
  },
  "net": {
    "ipv4": {
      "ip_forward": "1"
    }
  }
}
```

### 入力例 2

```bash
# システム設定
-kernel.hostname=myserver
kernel.max_files=10000
```

### 出力例 2

この設定ファイルを読み込むと、以下のようにJSON形式で出力されます。`-` が付いた `kernel.hostname` は、エラーが発生しても無視されます。

```json
{
  "kernel": {
    "max_files": "10000"
  }
}
```
