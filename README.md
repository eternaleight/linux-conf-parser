# Linux sysctl.conf パーサ（設定ファイルパーサ）

このRustプログラムは、`sysctl.conf` 形式に準拠した設定ファイルを解析し、ネストされたキーと値のペアを辞書型に格納してJSON形式で出力するパーサです。

## 機能

- **キーと値のペアを解析**: 設定ファイル内の `key=value` 形式の行をパースして、設定を辞書型に格納します。
- **ネストされたキーに対応**: ドット (`.`) で区切られたキー (`key.subkey.subsubkey=value`) をネストされたJSONオブジェクトに変換します。
- **コメントや空行を無視**: `#` や `;` で始まるコメント行、または空行を無視します。
- **エラー処理**: 行の先頭に `-` がある場合、設定の適用エラーが発生してもそのエラーを無視して処理を継続します。それ以外のエラーは標準エラー出力 (`stderr`) に出力されます。

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

このプログラムは指定された設定ファイルを読み込み、結果をJSON形式で標準出力に出力します。以下のコマンドで実行します：

```bash
cargo run
```

設定ファイルとして `config/sysctl.conf` を読み込み、その結果をJSON形式で出力します。

### 3. カスタマイズされたエラーハンドリング

- `-` で始まる行は、適用エラーが無視されます。
- 設定値の長さが4096文字を超える場合、警告が出力され、4096文字にトリムされます。

## 関数の説明

### `parse_line(line: &str) -> Option<(String, String, bool)>`

- 1行をパースし、`key=value` 形式の行であれば `Some((key, value, ignore_error))` を返します。`ignore_error` フラグは、行が `-` で始まっている場合に `true` となり、エラーが無視されます。コメント行や空行であれば `None` を返します。

### `set_nested_map(map: &mut serde_json::Map<String, serde_json::Value>, keys: &[&str], value: serde_json::Value, ignore_error: bool)`

- ドットで区切られたキー（例：`key.subkey.subsubkey=value`）をネストされたJSONオブジェクト形式に変換して `map` に挿入します。`ignore_error` フラグが `true` の場合、エラーが発生しても無視します。

### `parse_config(filename: &str) -> io::Result<serde_json::Map<String, serde_json::Value>>`

- 指定されたファイルを読み込み、各行を解析して辞書型に格納します。行の先頭に `-` がある場合、その行で発生したエラーは無視されます。

## 例

### 入力例 1

`以下の設定ファイル `config/sysctl.conf

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
