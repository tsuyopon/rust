# 概要
sample4と同じでディレクトリを切ってモジュールを分割する方法です。
しかし、少しだけ構成が異なります。 先程のsample4のgreet.rsをgreet直下にmod.rsとして移動しただけです。

greetに「mod.rs」という名称で配置することによってgreetのモジュールとして最初に読み込みが行われるようです。

```
├── greet
|    ├── mod.rs         // mod_aとmod_bはここから指定sample4のgreet.rsをgreet/mod.rsとしてリネームしただけ
│   ├── mod_a.rs
│   └── mod_b.rs
└── main.rs
```
