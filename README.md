# Warframe Bot

Warframe 查询机器人

# 命令

- /help - 显示帮助信息
- /arbitration - 仲裁
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

如果机器需要代理才能访问 TG，在 _TELOXIDE_PROXY_ 填入代理 url。

# 运行

需要安装 [rust](https://www.rust-lang.org/zh-CN/learn/get-started) 。

```bash
cargo run --release
```
# Docker 运行

```bash
# 编译
docker build -t warframe .
# 运行
docker run -d --name warframe --restart always warframe
# 更新翻译
docker exec -it warframe wget -q https://github.com/LJason77/WarframeBot/raw/master/locale/zh_CN/LC_MESSAGES/warframe.mo -O locale/zh_CN/LC_MESSAGES/warframe.mo
```
