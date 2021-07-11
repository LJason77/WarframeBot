//! 仲裁

use gettextrs::gettext;

use crate::api::{self, get_url};

pub async fn get_arbitration() -> String {
    // 读取缓存
    let (mut json, mut arbitration) =
        api::get_cache::<crate::models::arbitration::Arbitration>("arbitration").await;

    if api::need_update(&arbitration.expiry) {
        json = get_url("arbitration").await;
        arbitration = serde_json::from_str(&json).unwrap();
    }

    // 更新缓存
    api::update_cache(&json, "arbitration");

    format!(
        "地点：{}\n派系：{}\n任务：<strong>{}</strong>\n剩余：{}",
        api::get_node(&arbitration.node),
        &arbitration.enemy,
        gettext(&arbitration.mission_type),
        api::get_eta(&arbitration.expiry)
    )
}
