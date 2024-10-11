# ベンチマーク実行手順
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

プロジェクトディレクトリ内で `nightly` ツールチェーンを使用するように設定します。

```bash
rustup override set nightly
```

stable チャンネルに戻ります。


## 3. **結果の確認**

実行後、各ベンチマークの実行時間がナノ秒単位で表示されます。以下のような結果が表示されます。

```
test benchmarks::bench_parse_sysctl_conf      ... bench:     310,099.90 ns/iter (+/- 17,509.09)
test benchmarks::bench_parse_all_sysctl_files ... bench:     346,716.67 ns/iter (+/- 62,653.50)
test benchmarks::bench_empty_conf_file        ... bench:       2,000.00 ns/iter (+/- 50.00)
test benchmarks::bench_large_conf_file        ... bench:  1,000,000.00 ns/iter (+/- 100,000.00)
```

- **ns/iter**: 1回の処理にかかった時間（ナノ秒）。
- **(+/- XXX)**: 標準偏差。処理時間のばらつきを示します。

これで、プロジェクト内のベンチマークテストが実行できるようになります。

