## ライブラリクレート

### WIT WebAssembly Interface Type
インターフェース定義言語。
異なるプログラミング言語間でのインターフェースを定義するための仕様。

#### world 
WITではコンポーネント定義のことをworldと呼ぶ。
`export`で実装するインターフェースを指定。

WITの定義ファイルから生成されたクレートをRustで実装する。
クレートはsrc/bindings.rsに出力される。