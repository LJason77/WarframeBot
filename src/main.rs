#![deny(clippy::pedantic)]
#![allow(clippy::module_name_repetitions, clippy::non_ascii_literal)]

use std::{env, error::Error, fs, str::from_utf8};

use gettextrs::TextDomain;
use teloxide::{
    adaptors::DefaultParseMode,
    enable_logging,
    prelude2::{AutoSend, Bot, Message, Requester, RequesterExt},
    repls2::commands_repl,
    types::ParseMode,
    utils::command::BotCommand,
};

use api::{
    arbitration, event, fissures, invasion, new, nightwave, sortie, steel_path, syndicate, trader,
    worldstate,
};
use models::Command;

mod api;
mod models;

async fn answer(
    bot: DefaultParseMode<AutoSend<Bot>>,
    message: Message,
    command: Command,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    match command {
        Command::Help | Command::Start => {
            bot.send_message(message.chat.id, Command::descriptions()).await?
        }
        Command::Arbitration => {
            bot.send_message(message.chat.id, arbitration::get_arbitration().await).await?
        }
        Command::BountyCetus => {
            bot.send_message(message.chat.id, syndicate::get_cetus().await).await?
        }
        Command::BountyFortuna => {
            bot.send_message(message.chat.id, syndicate::get_fortuna().await).await?
        }
        Command::BountyNecralisk => {
            bot.send_message(message.chat.id, syndicate::get_necralisk().await).await?
        }
        Command::Events => bot.send_message(message.chat.id, event::get_event().await).await?,
        Command::Fissures => {
            bot.send_message(message.chat.id, fissures::get_fissures().await).await?
        }
        Command::Invasions => {
            bot.send_message(message.chat.id, invasion::get_invasion().await).await?
        }
        Command::News => bot.send_message(message.chat.id, new::get_new().await).await?,
        Command::Nightwave => {
            bot.send_message(message.chat.id, nightwave::get_nightwave().await).await?
        }
        Command::Sortie => bot.send_message(message.chat.id, sortie::get_sortie().await).await?,
        Command::SteelPath => {
            bot.send_message(message.chat.id, steel_path::get_reward().await).await?
        }
        Command::Trader => bot.send_message(message.chat.id, trader::get_trader().await).await?,
        Command::Worldstate => {
            bot.send_message(message.chat.id, worldstate::get_world().await).await?
        }
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
            println!("语言已找到： {:?}", from_utf8(&locale.expect("语言未找到")).unwrap())
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

    commands_repl(bot, answer, Command::ty()).await;
}
