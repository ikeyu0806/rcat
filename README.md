rust試したくて作ったcatみたいなCLIツール

## プロジェクト立ち上げ
```
docker-compose build
docker-compose run rcat cargo init
docker-compose run rcat cargo add clap
```

## 実行
```
docker-compose run rcat cargo run README.md
# 行番号出力
docker-compose run rcat cargo run README.md -n
```
