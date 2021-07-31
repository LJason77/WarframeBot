#![deny(clippy::pedantic)]
#![allow(clippy::module_name_repetitions, clippy::non_ascii_literal)]

use std::{env, error::Error, fs, str::from_utf8, sync::Arc};

use gettextrs::TextDomain;
use teloxide::{
    adaptors::DefaultParseMode, commands_repl, enable_logging, prelude::*,
    types::ParseMode, utils::command::BotCommand,
};

use api::{
    arbitration, event, fissures, invasion, new, nightwave, sortie, steel_path,
    syndicate, trader, worldstate,
};
use models::Command;

mod api;
mod models;

async fn answer(
    cx: UpdateWithCx<Arc<DefaultParseMode<AutoSend<Bot>>>, Message>,
    command: Command,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    match command {
        Command::Help | Command::Start => {
            cx.answer(Command::descriptions()).send().await?
        }
        Command::Arbitration => {
            cx.answer(arbitration::get_arbitration().await)
                .send()
                .await?
        }
        Command::BountyCetus => cx.answer(syndicate::get_cetus().await).send().await?,
        Command::BountyFortuna => {
            cx.answer(syndicate::get_fortuna().await).send().await?
        }
        Command::BountyNecralisk => {
            cx.answer(syndicate::get_necralisk().await).send().await?
        }
        Command::Events => cx.answer(event::get_event().await).send().await?,
        Command::Fissures => cx.answer(fissures::get_fissures().await).send().await?,
        Command::Invasions => cx.answer(invasion::get_invasion().await).send().await?,
        Command::News => cx.answer(new::get_new().await).send().await?,
        Command::Nightwave => cx.answer(nightwave::get_nightwave().await).send().await?,
        Command::Sortie => cx.answer(sortie::get_sortie().await).send().await?,
        Command::SteelPath => cx.answer(steel_path::get_reward().await).send().await?,
        Command::Trader => cx.answer(trader::get_trader().await).send().await?,
        Command::Worldstate => cx.answer(worldstate::get_world().await).send().await?,
    };

    Ok(())
}

#[allow(clippy::semicolon_if_nothing_returned)]
#[tokio::main]
async fn main() {
    // 导入环境变量
    dotenv::dotenv().ok();
    // 移除缓存目录
    fs::remove_dir_all("cache").ok();
    // 创建缓存目录
    fs::create_dir_all("cache").ok();

    match TextDomain::new("warframe")
        .locale("zh_CN.UTF-8")
        .prepend(env::current_dir().unwrap().to_str().unwrap())
        .skip_system_data_paths()
        .init()
    {
        Ok(locale) => {
            println!(
                "语言已找到： {:?}",
                from_utf8(&locale.expect("语言未找到")).unwrap()
            )
        }
        Err(error) => {
            panic!("语言未找到： {:?}", error)
        }
    };

    run().await;
}

async fn run() {
    enable_logging!();
    log::info!("启动 Warframe Bot...");

    let bot = Bot::from_env().auto_send().parse_mode(ParseMode::Html);

    let bot_name: String = "Warframe Bot".to_owned();
    commands_repl(Arc::new(bot), bot_name, answer).await;
}
