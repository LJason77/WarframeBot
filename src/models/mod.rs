use teloxide::utils::command::BotCommand;

pub mod invasion;
pub mod nightwave;
pub mod sortie;

#[derive(BotCommand)]
#[command(rename = "lowercase", description = "当前支持下列查询：")]
pub enum Command {
	#[command(description = "显示帮助信息")]
	Help,
	#[command(description = "入侵")]
	Invasions,
	#[command(description = "午夜电波")]
	Nightwave,
	#[command(description = "突击")]
	Sortie,
}
