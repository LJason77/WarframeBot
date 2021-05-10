use std::{fs, sync::Arc};

use teloxide::{prelude::*, utils::command::BotCommand};

use crate::models::Command;

mod api;
mod models;

async fn answer(
	cx: UpdateWithCx<Arc<teloxide::adaptors::DefaultParseMode<AutoSend<Bot>>>, Message>,
	command: Command,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
	match command {
		Command::Help => cx.answer(Command::descriptions()).send().await?,
		Command::Arbitration => {
			cx.answer(api::arbitration::get_arbitration().await)
				.send()
				.await?
		}
		Command::BountyCetus => cx.answer(api::syndicate::get_cetus().await).send().await?,
		Command::BountyFortuna => cx.answer(api::syndicate::get_fortuna().await).send().await?,
		Command::BountyNecralisk => cx.answer(api::syndicate::get_necralisk().await).send().await?,
		Command::Events => cx.answer(api::event::get_event().await).send().await?,
		Command::Fissures => cx.answer(api::fissures::get_fissures().await).send().await?,
		Command::Invasions => cx.answer(api::invasion::get_invasion().await).send().await?,
		Command::News => cx.answer(api::new::get_new().await).send().await?,
		Command::Nightwave => cx.answer(api::nightwave::get_nightwave().await).send().await?,
		Command::Sortie => cx.answer(api::sortie::get_sortie().await).send().await?,
		Command::SteelPath => cx.answer(api::steel_path::get_reward().await).send().await?,
		Command::Trader => cx.answer(api::trader::get_trader().await).send().await?,
		Command::Worldstate => cx.answer(api::worldstate::get_world().await).send().await?,
	};

	Ok(())
}

#[tokio::main]
async fn main() {
	// 导入环境变量
	dotenv::dotenv().ok();
	// 移除缓存目录
	fs::remove_dir_all("cache").unwrap();
	// 创建缓存目录
	fs::create_dir_all("cache").unwrap();

	gettextrs::TextDomain::new("warframe")
		.locale("zh_CN.UTF-8")
		.prepend(std::env::current_dir().unwrap().to_str().unwrap())
		.skip_system_data_paths()
		.init()
		.unwrap();

	run().await;
}

async fn run() {
	teloxide::enable_logging!();
	log::info!("Starting Warframe_Bot...");

	let bot = Bot::from_env()
		.auto_send()
		.parse_mode(teloxide::types::ParseMode::Html);

	let bot_name: String = "Warframe Bot".to_owned();
	teloxide::commands_repl(Arc::new(bot), bot_name, answer).await;
}
