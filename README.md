# Warframe Bot

Warframe 查询机器人

# 命令

- /help - 显示帮助信息
- /arbitration - 仲裁
- /events - 活动
- /fissures - 裂缝
- /invasions - 入侵
- /news - 新闻
- /nightwave 午夜电波
- /sortie - 突击
- /trader - 虚空商人
- /worldstate - 世界状态

# 配置

将 `.env.example` 文件复制为 `.env`，并在 _TELOXIDE_TOKEN_ 填入 TG 机器人的 token。

如果机器需要代理才能访问 TG，在 _TELOXIDE_PROXY_ 填入代理 url。

# 运行

需要安装 [rust](https://www.rust-lang.org/zh-CN/learn/get-started) 。

```bash
cargo run --release
```
# Docker 运行

已有计划，待执行。 