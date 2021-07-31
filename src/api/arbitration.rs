//! 仲裁

use gettextrs::gettext;

use crate::models::arbitration::Arbitration;

use super::{get_cache, get_eta, get_node, get_url, need_update};

pub async fn get_arbitration() -> String {
    // 读取缓存
    let mut arbitration = match get_cache::<Arbitration>("arbitration").await {
        Ok(arbitration) => arbitration,
        Err(err) => return err,
    };

    if need_update(&arbitration.expiry) {
        arbitration = match get_url::<Arbitration>("arbitration", None).await {
            Ok(arbitration) => arbitration,
            Err(err) => return err,
        };
    }

    format!(
        "地点：{}\n派系：{}\n任务：<strong>{}</strong>\n剩余：{}",
        get_node(&arbitration.node),
        &arbitration.enemy,
        gettext(&arbitration.mission_type),
        get_eta(&arbitration.expiry)
    )
}
