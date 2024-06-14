# 言語処理系論 中間レポート

> 四則演算と変数定義から成る関数電卓言語を実装可能なスタックマシンの命令セットを設計し，具体的な翻訳例を使って説明せよ．

```
$ cargo run
Mode (1: interpreter, 2: vm): 2
> 1 + 2 * 3
000: ldc   1
001: ldc   2
002: ldc   3
003: mul
004: add
= 7
> a = 2 * 3 + 1 / 2
000: ldc   2
001: ldc   3
002: mul
003: ldc   1
004: ldc   2
005: div
006: add
007: stl   a
> a
000: ldl   a
= 6.5
> b = sqrt(6.5 + 2.5)
000: ldc   6.5
001: ldc   2.5
002: add
003: call  sqrt
004: stl   b
> -(b - 1)^3!
000: ldl   b
001: ldc   1
002: sub
003: ldc   3
004: fact
005: pow
006: neg
= -64
> ^C
```
