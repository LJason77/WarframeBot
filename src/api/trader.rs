//! 虚空商人

use gettextrs::gettext;

use crate::api;

pub async fn get_trader() -> String {
	// 读取缓存
	let (mut json, mut trader) = api::get_cache::<crate::models::trader::Trader>("voidTrader").await;

	if api::need_update(&trader.activation) && !trader.active || api::need_update(&trader.expiry) {
		json = api::get_url("voidTrader").await;
		trader = serde_json::from_str(&json).unwrap();
	}

	// 更新缓存
	api::update_cache(&json, "voidTrader");

	let mut trader_str = format!("{}\n", api::get_node(&trader.location));
	if !trader.active {
		trader_str.push_str(format!("距离虚空商人抵达：{}", api::get_eta(&trader.activation)).as_str());
	} else {
		let mut items = String::new();
		for item in trader.inventory {
			items.push_str(
				format!(
					"<strong>{}</strong>\n金币：{}  |  现金：{}\n\n",
					gettext(item.item),
					item.ducats,
					item.credits
				)
				.as_str(),
			);
		}
		trader_str
			.push_str(format!("距离虚空商人离开：{}\n\n{}", api::get_eta(&trader.expiry), items).as_str());
	}

	trader_str
}
