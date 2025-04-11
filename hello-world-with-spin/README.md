## echo サーバーの起動
`$spin build`でビルド。`$spin up`で起動。

## glitch art API サーバーへのリクエスト
`$curl -X POST -H "Content-Type: image/png" --data-binary @woop_org.png --output glitch-art.png http://localhost:3000/glitch`
```
  % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
                                 Dload  Upload   Total   Spent    Left  Speed
100 41676    0 22981  100 18695  2276k  1851k --:--:-- --:--:-- --:--:-- 4522k
```