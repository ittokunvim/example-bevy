# Bevy games

このリポジトリは、RustのBevyを使って作成したゲームのソースコードを置く場所です

### ゲーム

**タイミングゲーム**

真ん中にキューをタイミングよく合わせて高得点を狙うゲームです。

以下のコマンドを実行することで、ゲームを開始します。

```bash
cargo run --example timing
```

[ソースコード](https://github.com/ittokun/bevy-games/blob/main/examples/timing.rs)

**クリックゲーム**

画面内を跳ね返っているボールをクリックして、どんどんボールを消していくゲームです。

以下のコマンドを実行することで、ゲームを開始します。

```bash
cargo run --example click
```

**2Dシューティングゲーム**

画面上部にいる敵を弾を打って倒すゲームです。

以下のコマンドを実行することで、ゲームを開始します。

```bash
cargo run --example 2d_shooting
```
