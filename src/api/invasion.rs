use gettextrs::gettext;

use crate::api;

pub async fn get_invasion() -> String {
	let json = api::get_url("invasions").await;
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
					"<strong>{}  |  {:.2}%  |  {}</strong>\n{}   |   {}\n\n",
					gettext(invasion.desc),
					invasion.completion,
					api::get_node(&invasion.node),
					attacker_reward,
					defender_reward
				)
				.as_str(),
			)
		}
	}

	invasions_str
}
