## 学んだことをつらつらメモる

## 関連関数
::new()
オブジェクト自体(インスタンスではなく)に関連づけられている関数。
　→Javaで言うスタティックメソッド

## use
use std::io
ioの関連関数を呼び出している。
この記述がなくても省略しなければ
関連関数の呼び出しができる。
```
std::io::stdin()
```

## let mut
```
let foo = 5; // <= イミュータブル
let mut bar = 5; // <= ミュータブル
```
Rustの束縛は基本イミュータブル。
　→不変という意味
　→変数定義のことを束縛と呼んでいる

## match構文


## cargoについて
cargoは

## stdについて

### stdin()
入力を受け取る。

### cmp()
比較可能な物を比較する。


## 出会ったエラー

```
error[E0423]: expected value, found module `thread`
  --> src/main.rs:39:9
   |
39 |         thread.sleep(Duration::from_millis(1000));
   |         ^^^^^^-----------------------------------
   |         |
   |         did you mean `thread::sleep(...)`?
```

structの束縛呼び出しをメソッドのように扱うと起きる。


## その他用語
### FFI

### GIL
 ‘global interpreter lock’ (GIL),