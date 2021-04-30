use teloxide::{prelude::*, utils::command::BotCommand};

use crate::models::Command;

mod api;
mod models;

async fn answer(
	cx: UpdateWithCx<AutoSend<Bot>, Message>,
	command: Command,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
	match command {
		Command::Help => cx.answer(Command::descriptions()).send().await?,
		Command::Events => cx.answer(api::event::get_event().await).send().await?,
		Command::Invasions => cx.answer(api::invasion::get_invasion().await).send().await?,
		Command::Nightwave => cx.answer(api::nightwave::get_nightwave().await).send().await?,
		Command::Sortie => cx.answer(api::sortie::get_sortie().await).send().await?,
		Command::Trader => cx.answer(api::trader::get_trader().await).send().await?,
	};

	Ok(())
}

#[tokio::main]
async fn main() {
	dotenv::dotenv().ok();

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

	let bot = Bot::from_env().auto_send();

	let bot_name: String = "Warframe Bot".to_owned();
	teloxide::commands_repl(bot, bot_name, answer).await;
}
