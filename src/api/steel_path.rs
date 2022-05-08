//! 钢铁之路奖励

use gettextrs::gettext;

use crate::models::SteelPath;

use super::{get_cache, get_eta, get_url, need_update};

pub async fn get_reward() -> String {
    // 读取缓存
    let mut steel_path = match get_cache::<SteelPath>("steelPath").await {
        Ok(steel_path) => steel_path,
        Err(err) => return err,
    };

    if need_update(&steel_path.expiry) {
        steel_path = match get_url("steelPath", None).await {
            Ok(steel_path) => steel_path,
            Err(err) => return err,
        };
    }

    format!(
        "物品：<strong>{}</strong>\n价格：<strong>{}</strong>\n剩余：<strong>{}</strong>",
        gettext(&steel_path.current_reward.name),
        &steel_path.current_reward.cost,
        get_eta(&steel_path.expiry)
    )
}
