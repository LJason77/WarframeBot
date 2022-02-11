use teloxide::utils::command::BotCommand;

pub mod arbitration;
pub mod event;
pub mod fissures;
pub mod invasion;
pub mod new;
pub mod nightwave;
pub mod sortie;
pub mod steel_path;
pub mod syndicate;
pub mod trader;
pub mod worldstate;

#[derive(BotCommand, Clone)]
#[command(rename = "lowercase", description = "当前支持下列查询：")]
pub enum Command {
    #[command(description = "off")]
    Start,
    #[command(description = "显示帮助信息")]
    Help,
    #[command(description = "仲裁(不稳定)")]
    Arbitration,
    #[command(description = "希图斯赏金")]
    BountyCetus,
    #[command(description = "福尔图娜赏金")]
    BountyFortuna,
    #[command(description = "殁世幽都赏金")]
    BountyNecralisk,
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
    #[command(description = "钢铁之路奖励")]
    SteelPath,
    #[command(description = "虚空商人")]
    Trader,
    #[command(description = "世界状态")]
    Worldstate,
}

pub struct Header {
    pub key: &'static str,
    pub value: &'static str,
}
