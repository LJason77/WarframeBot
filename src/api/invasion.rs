//! 入侵

use gettextrs::gettext;

use crate::models::invasion::Invasion;

use super::{get_node, get_url};

pub async fn get_invasion() -> String {
    let invasions = match get_url::<Vec<Invasion>>("invasions", None).await {
        Ok(invasions) => invasions,
        Err(err) => return err,
    };

    let mut invasions_str = String::new();
    for invasion in invasions {
        if !invasion.completed {
            // 入侵奖励
            let attacker_reward = if invasion.attacker_reward.counted_items.is_empty() {
                "无".to_owned()
            } else {
                format!(
                    "{} ({})",
                    gettext(&invasion.attacker_reward.counted_items[0].key),
                    &invasion.attacker_reward.counted_items[0].count
                )
            };
            // 防守奖励
            let defender_reward = if invasion.defender_reward.counted_items.is_empty() {
                "无".to_owned()
            } else {
                format!(
                    "{} ({})",
                    gettext(&invasion.defender_reward.counted_items[0].key),
                    &invasion.defender_reward.counted_items[0].count
                )
            };
            invasions_str.push_str(
                format!(
                    "<strong>{}  |  {:.2}%  |  {}</strong>\n{}   |   {}\n\n",
                    gettext(invasion.desc),
                    invasion.completion,
                    get_node(&invasion.node),
                    attacker_reward,
                    defender_reward
                )
                .as_str(),
            );
        }
    }

    invasions_str
}
