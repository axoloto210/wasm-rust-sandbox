## greetクレートを使用

### anyhow
anyhowは「どんなエラーでも扱える」というコンセプトのエラーハンドリングライブラリ。
`Result<T, anyhow::Error>`を`anyhow::Result<T>`と書いたりできる。