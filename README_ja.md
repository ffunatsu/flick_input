# flick_input

[English](README.md) | [日本語](README_ja.md)

スマホのフリック入力を、PCの外付けキーボードのように使えるツールです。

## クイックスタート

### 1. ダウンロードと起動

**[GitHub Releases](https://github.com/ffunatsu/flick_input/releases)** からお使いのOS向けのバイナリをダウンロードして実行します。

- **Windows**: `flick_input.exe` をダブルクリック
- **macOS / Linux**: ターミナルで `./flick_input` を実行

もしくはソースから直接起動：
```bash
cargo run --release
```

### 2. スマホから接続

1. PCとスマホを同じWi-Fi（ローカルネットワーク）に接続します。
2. PC画面（ターミナル）に表示されたQRコードをスマホのカメラで読み取ります（またはURLをブラウザで直接開きます）。

### 3. 入力する

1. スマホ画面の入力欄をタップし、フリック等で文字を入力します。
2. **送信**ボタン（またはEnter）を押すと、PCで現在アクティブなウィンドウのカーソル位置に文字が入力されます。

---

## 各OSの注意点

- **Windows**: 設定不要ですぐに使えます。
- **macOS**: アプリを実行するターミナル等に「アクセシビリティ」権限が必要です（**システム設定 > プライバシーとセキュリティ > アクセシビリティ**）。
- **Linux**: X11またはWaylandの適切な権限が必要です。

---

## 仕組み

1. PC側で軽量なローカルWebサーバーが起動します。
2. スマホは専用アプリ不要で、ブラウザからWeb UIを開きます。
3. 確定した文字列が非同期でPCに送られ、PC側でキー入力として自動注入されます。

---

## ソースコードからのビルド

```bash
git clone https://github.com/ffunatsu/flick_input.git
cd flick_input
cargo build --release
```

バイナリは `target/release/flick_input` に生成されます。

## ライセンス

0BSD
