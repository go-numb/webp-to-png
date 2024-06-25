# webp-to-png
【ディレクトリのwebpをpngに変更し、webpを削除
```sh
$ Get-ChildItem ./ -Filter *.webp | ForEach-Object {ffmpeg -i $_.FullName -vcodec png "$($_.Name).png"; rm "$($_.Name)"}
 ```

 上記シェルをRustで実現。