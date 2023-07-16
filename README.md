# Bevy games

このリポジトリには、RustのBevyを使って作成したゲームのソースコードが置かれています。

このソースコードを以下に記されているコマンドを実行することによって、遊ぶことができるようになっています。

よければプレイしてみてください。

### タイミングゲーム

真ん中にキューをタイミングよく合わせて高得点を狙うゲームです。

以下のコマンドを実行することで、ゲームを開始します。

```bash
cargo run --example timing_game
```

[ソースコード](https://github.com/ittokunvim/bevy-games/blob/main/examples/timing.rs)

### クリックゲーム

画面内を跳ね返っているボールをクリックして、どんどんボールを消していくゲームです。

以下のコマンドを実行することで、ゲームを開始します。

```bash
cargo run --example click_game
```

[ソースコード](https://github.com/ittokunvim/bevy-games/blob/main/examples/click.rs)

### 2Dシューティングゲーム

画面上部にいる敵を弾を打って倒すゲームです。

以下のコマンドを実行することで、ゲームを開始します。

```bash
cargo run --example 2d_shooting
```

[ソースコード](https://github.com/ittokunvim/bevy-games/blob/main/examples/2d_shooting.rs)

### フロッガー

カエル（プレイヤー）が危険な道を渡り、ゴールに辿り着くというゲームです。

以下のコマンドを実行することで、ゲームを開始します。

```bash
cargo run --example frogger
```

[ソースコード](https://github.com/ittokunvim/bevy-games/blob/main/examples/frogger_game.rs)

### フラッピーバード

画面のタップによって画面上を飛ぶ鳥の高さを調整し、土管の隙間をぶつけずに飛ばし続けるゲームです。

以下のコマンドを実行することで、ゲームを開始します。

```bash
cargo run --example flappy_bird
```

[ソースコード](https://github.com/ittokunvim/bevy-games/blob/main/examples/flappy_bird.rs)
