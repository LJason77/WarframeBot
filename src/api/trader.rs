//! 虚空商人

use std::{fs::File, io::Write, path::Path};

use gettextrs::gettext;

use crate::api;

pub async fn get_trader() -> String {
	// 读取缓存
	let mut json = api::get_cache("voidTrader").await;
	let mut trader: crate::models::trader::Trader = serde_json::from_str(&json).unwrap();

	if api::need_update(&trader.activation) && !trader.active || api::need_update(&trader.expiry) {
		json = api::get_url("voidTrader").await;
		trader = serde_json::from_str(&json).unwrap();
	}

	// 更新缓存
	let json_file = Path::new("cache/voidTrader.json");
	let mut file = match File::create(&json_file) {
		Err(error) => panic!("无法创建 {}：{}", json_file.display(), error),
		Ok(file) => file,
	};
	if let Err(why) = file.write_all(&json.as_bytes()) {
		println!("更新缓存失败：{}", why)
	}

	let mut trader_str = format!("{}\n", gettext(trader.location));
	if !trader.active {
		trader_str.push_str(format!("距离虚空商人抵达：{}", api::get_eta(&trader.activation)).as_str());
	} else {
		let mut items = String::new();
		for item in trader.inventory {
			items.push_str(
				format!(
					"{}\n金币：{}\n现金：{}\n\n",
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
