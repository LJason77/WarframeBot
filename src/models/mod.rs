use teloxide::utils::command::BotCommand;

pub mod arbitration;
pub mod event;
pub mod fissures;
pub mod invasion;
pub mod new;
pub mod nightwave;
pub mod sortie;
pub mod trader;

#[derive(BotCommand)]
#[command(rename = "lowercase", description = "当前支持下列查询：")]
pub enum Command {
	#[command(description = "显示帮助信息")]
	Help,
	#[command(description = "仲裁")]
	Arbitration,
	#[command(description = "活动")]
	Events,
	#[command(description = "裂缝")]
	Fissures,
	#[command(description = "入侵")]
	Invasions,
	#[command(description = "新闻")]
	News,
	#[command(description = "午夜电波")]
	Nightwave,
	#[command(description = "突击")]
	Sortie,
	#[command(description = "虚空商人")]
	Trader,
}
