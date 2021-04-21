use gettextrs::gettext;

use crate::api;

pub async fn get_invasion() -> String {
	let json = api::get_url("https://api.warframestat.us/pc/invasions").await;
	let invasions: Vec<crate::models::invasion::Invasion> = serde_json::from_str(&json).unwrap();

	let mut invasions_str = String::new();
	for invasion in invasions {
		if !invasion.completed {
			// 入侵奖励
			let attacker_reward = if invasion.attacker_reward.counted_items.len() > 0 {
				format!(
					"{} ({})",
					gettext(&invasion.attacker_reward.counted_items[0].key),
					&invasion.attacker_reward.counted_items[0].count
				)
			} else {
				"无".to_owned()
			};
			// 防守奖励
			let defender_reward = if invasion.defender_reward.counted_items.len() > 0 {
				format!(
					"{} ({})",
					gettext(&invasion.defender_reward.counted_items[0].key),
					&invasion.defender_reward.counted_items[0].count
				)
			} else {
				"无".to_owned()
			};
			invasions_str.push_str(
				format!(
					"{} ({:.2}%)\n入侵节点：{}\n剩余时间：{}\n------------\n入侵阵营：{}\n入侵奖励：{}\n------------\n防守阵营：{}\n防守奖励：{}\n\n",
					gettext(invasion.desc),
					invasion.completion,
					api::get_node(&invasion.node),
					invasion.eta,
					invasion.attacking_faction,
					attacker_reward,
					invasion.defending_faction,
					defender_reward
				)
				.as_str(),
			)
		}
	}

	invasions_str
}
