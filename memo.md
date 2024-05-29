# 知ったこと

- Stringにmatchは使えない．matchしたいときは&str型に変換(Stringの変数に&つけるだけ)
- モジュール分割は，main.rsに"mod ファイル名" 使いたい所で "use crate::ファイル名"みたいな感じにする

## ソート
ベクタに保持した構造体をソートするときはsort_byを使う．()

## コマンドライン引数
std::env::args()を使う
ベクタとして扱いたいときはcollect()，Option型で取り出したいときは.nth()を使う
