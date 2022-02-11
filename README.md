# Warframe Bot

[![build badge](https://github.com/LJason77/WarframeBot/actions/workflows/rust.yml/badge.svg?branch=master)](https://github.com/LJason77/WarframeBot/actions/workflows/rust.yml)
![GitHub forks](https://img.shields.io/github/forks/LJason77/WarframeBot?style=social)
![GitHub Repo stars](https://img.shields.io/github/stars/LJason77/WarframeBot?style=social)

Warframe 查询机器人

# 命令

- /help - 显示帮助信息
- /arbitration - 仲裁(不稳定)
- /bountycetus - 希图斯赏金
- /bountyfortuna - 福尔图娜赏金
- /bountynecralisk - 殁世幽都赏金
- /events - 活动
- /fissures - 裂缝
- /invasions - 入侵
- /news - 新闻
- /nightwave 午夜电波
- /sortie - 突击
- /steelpath - 钢铁之路奖励
- /trader - 虚空商人
- /worldstate - 世界状态

# 配置

将 `.env.example` 文件复制为 `.env`，并在 _TELOXIDE_TOKEN_ 填入 TG 机器人的 token。

如果机器需要代理才能访问 TG，取消 _TELOXIDE_PROXY_ 的注释并填入代理 url，如 `http://127.0.0.1:8123`。

# 运行

需要安装 [rust](https://www.rust-lang.org/zh-CN/learn/get-started) 。

```shell
cargo run --release
```

# Docker 运行

```shell
# 编译
docker build -t warframe .
# 运行
docker run -d --name warframe --restart always warframe
# 更新翻译
docker exec -it warframe wget -q https://github.com/LJason77/WarframeBot/raw/master/locale/zh_CN/LC_MESSAGES/warframe.mo -O locale/zh_CN/LC_MESSAGES/warframe.mo
```

# 自行翻译文件

更新 `zh_CN.po` 文件后，运行：

```shell
msgfmt zh_CN.po -o locale/zh_CN/LC_MESSAGES/warframe.mo
```

## 许可

[![996.icu](https://img.shields.io/badge/link-996.icu-red.svg)](https://996.icu)
[![LICENSE](https://img.shields.io/badge/license-Anti%20996-blue.svg)](https://github.com/996icu/996.ICU/blob/master/LICENSE)
![GitHub](https://img.shields.io/github/license/LJason77/WarframeBot)
