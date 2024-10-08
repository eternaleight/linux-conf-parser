# Linux sysctl.confファイパーサ（設定ファイルパーサ）

このRustプログラムは、`key=value` 形式の設定ファイルを解析する簡単なパーサです。設定ファイルを読み込み、各行を処理して、JSON形式で結果を出力します。

## 機能

- **キーと値のペアを解析**: 設定ファイルの各行を読み込み、`key=value` 形式の行を解析します。
- **ネストされたキーに対応**: ドット (`.`) で区切られたキー (例: `key.subkey.subsubkey=value`) をネストされたJSONオブジェクトに変換します。
- **コメントや空行を無視**: `#` や `;` で始まるコメント行、または空行を無視します。
- **エラー処理**: 読み込みエラーが発生した場合には、標準エラー出力 (`stderr`) にエラーメッセージを出力しつつ、処理を継続します。

## 使用方法

### 1. 設定ファイルのフォーマット

設定ファイルは、以下の形式で記述されます：

```bash
# コメント行
key1=value1
key2.subkey=value2
key3.subkey1.subkey2=value3

; こちらもコメント行
```

- `#` や `;` で始まる行はコメントとして無視されます。
- `key=value` の形式で記述された行はキーと値として認識され、ネストされたキーもサポートされています。

### 2. プログラムの実行

このプログラムは、指定された設定ファイルを読み込み、JSON形式で出力します。以下のコマンドで実行します：

```bash
cargo run
```

設定ファイルとして `config/sysctl.conf` を読み込み、結果を標準出力にJSON形式で出力します。

### 3. エラーハンドリング

エラーが発生した場合は、標準エラー出力 (`stderr`) にエラーメッセージが表示されます。エラーが出た行を無視しつつ、他の行の処理は続行されます。

## 関数の説明

### `parse_line(line: &str) -> Option<(String, String)>`

- 1行をパースし、`key=value` の形式であれば `Some((key, value))` を返します。コメント行や空行であれば `None` を返します。

### `set_nested_map(map: &mut serde_json::Map<String, serde_json::Value>, keys: &[&str], value: serde_json::Value)`

- ネストされたキー (`key.subkey.subsubkey=value`) を対応するJSONオブジェクトの形式に変換して `map` に挿入します。

### `parse_config(filename: &str) -> io::Result<serde_json::Map<String, serde_json::Value>>`

- 指定されたファイルを読み込み、各行を解析してネストされたマップに変換します。

## 例

以下の設定ファイル `config/sysctl.conf`:
### 入力例
```bash
# システム設定
kernel.hostname=myserver
kernel.max_files=10000
net.ipv4.ip_forward=1
```

この設定ファイルを読み込むと、以下のようにJSON形式で出力されます：
### 出力例
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
