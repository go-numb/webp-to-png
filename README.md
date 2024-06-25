# webp-to-png
【ディレクトリのwebpをpngに変更し、webpを削除
```sh
$ Get-ChildItem ./ -Filter *.webp | ForEach-Object {ffmpeg -i $_.FullName -vcodec png "$($_.Name).png"; rm "$($_.Name)"}
 ```

 上記シェルをRustで実現。

 ## Futures
 - 処理時間目安: 1500MB(50dirs, 3100files) -> 3.0413144s