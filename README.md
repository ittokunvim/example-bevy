# Example Bevy

ここはRustのBevyで作成したゲームが置かれているリポジトリです。

## ゲーム

以下のリストには作成したゲームと遊ぶ手順が記されています。

### タイミングゲーム

真ん中にキューをタイミングよく合わせて高得点を狙うゲーム。

```bash
# 遊ぶ
cargo run --example timing_game
# Wasmに変換
cargo make timing_game
```

### クリックゲーム

画面内を跳ね返っているボールをクリックして、ボールを全消しを目指すゲーム。

```bash
# 遊ぶ
cargo run --example click_game
# Wasmに変換
cargo make click_game
```

### 2Dシューティングゲーム

画面上部にいる敵を弾を打って倒すゲーム。

```bash
# 遊ぶ
cargo run --example 2d_shooting
# Wasmに変換
cargo make 2d_shooting
```

### フロッガー

プレイヤー（カエル）を障害物を避けつつ対岸にあるゴールを目指すゲーム。

```bash
# 遊ぶ
cargo run --example frogger
# Wasmに変換
cargo make frogger
```
### フラッピーバード

プレイヤー（鳥）を操作して、障害物を避けて飛ばし続けて高得点を目指すゲーム。

```bash
# 遊ぶ
cargo run --example flappy_bird
# Wasmに変換
cargo make flappy_bird
```

### キャッチゲーム

落ちてくるオブジェクトをキャッチして高得点を目指すゲーム。

```bash
# 遊ぶ
cargo run --example catch_game
# Wasmに変換
cargo make catch_game
```
### ランアンドジャンプ

プレイヤーを操作して障害物を回避してゴールを目指すゲーム。

```bash
# 遊ぶ
cargo run --example run_and_jump
# Wasmに変換
cargo make run_and_jump
```