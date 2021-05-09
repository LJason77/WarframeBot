//! 钢铁之路奖励

use crate::api;

pub async fn get_reward() -> String {
	// 读取缓存
	let (mut json, mut steel_path) =
		api::get_cache::<crate::models::steel_path::SteelPath>("steelPath").await;

	if api::need_update(&steel_path.expiry) {
		json = api::get_url("steelPath").await;
		steel_path = serde_json::from_str(&json).unwrap();
	}

	// 更新缓存
	api::update_cache(&json, "steelPath");

	format!(
		"物品：<strong>{}</strong>\n价格：<strong>{}</strong>\n剩余：<strong>{}</strong>",
		gettextrs::gettext(&steel_path.current_reward.name),
		&steel_path.current_reward.cost,
		api::get_eta(&steel_path.expiry)
	)
}
