//! 警报

use gettextrs::gettext;

use crate::models::alert::Alert;

use super::{get_cache, get_eta, get_node, get_url, need_update};

pub async fn get_alerts() -> String {
    // 读取缓存
    let mut alerts = match get_cache::<Vec<Alert>>("alerts").await {
        Ok(alerts) => alerts,
        Err(err) => return err,
    };

    for alert in &alerts {
        if need_update(&alert.expiry) {
            alerts = match get_url::<Vec<Alert>>("alerts", None).await {
                Ok(alerts) => alerts,
                Err(err) => return err,
            };
            break;
        }
    }

    let mut output = String::new();
    for alert in alerts {
        let s = format!(
            "地点：{}\n派系：{}\n任务：<strong>{}</strong>\n奖励：{}\n剩余：{}\n\n",
            get_node(&alert.mission.node),
            &alert.mission.faction,
            gettext(&alert.mission.mission_type),
            gettext(&alert.mission.reward.items),
            get_eta(&alert.expiry)
        );
        output += &s;
    }
    output
}
