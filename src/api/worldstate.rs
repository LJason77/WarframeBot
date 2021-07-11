//! 世界状态

use gettextrs::gettext;

use crate::api;
use crate::models::worldstate::{self, EarthCycle};

pub async fn get_world() -> String {
    // 读取缓存
    let (mut json_earth, mut earth) = api::get_cache::<EarthCycle>("earthCycle").await;
    let (mut json_cetus, mut cetus) = api::get_cache::<EarthCycle>("cetusCycle").await;
    let (mut json_vallis, mut vallis) =
        api::get_cache::<worldstate::VallisCycle>("vallisCycle").await;
    let (mut json_cambion, mut cambion) =
        api::get_cache::<worldstate::CambionCycle>("cambionCycle").await;

    if api::need_update(&earth.expiry) {
        json_earth = api::get_url("earthCycle").await;
        earth = serde_json::from_str(&json_earth).unwrap();
    }
    if api::need_update(&cetus.expiry) {
        json_cetus = api::get_url("cetusCycle").await;
        cetus = serde_json::from_str(&json_cetus).unwrap();
    }
    if api::need_update(&vallis.expiry) {
        json_vallis = api::get_url("vallisCycle").await;
        vallis = serde_json::from_str(&json_vallis).unwrap();
    }
    if api::need_update(&cambion.expiry) {
        json_cambion = api::get_url("cambionCycle").await;
        cambion = serde_json::from_str(&json_cambion).unwrap();
    }

    // 更新缓存
    api::update_cache(&json_earth, "earthCycle");
    api::update_cache(&json_cetus, "cetusCycle");
    api::update_cache(&json_vallis, "vallisCycle");
    api::update_cache(&json_cambion, "cambionCycle");

    let mut world_str = String::new();

    // 地球昼夜
    world_str.push_str(
        format!(
            "地球：<strong>{}</strong>\n时间：{}\n\n",
            gettext(&earth.state),
            api::get_eta(&earth.expiry)
        )
        .as_str(),
    );
    // 希图斯昼夜
    world_str.push_str(
        format!(
            "希图斯：<strong>{}</strong>\n时间：{}\n\n",
            gettext(&cetus.state),
            api::get_eta(&cetus.expiry)
        )
        .as_str(),
    );
    // 奥布山谷
    world_str.push_str(
        format!(
            "奥布山谷：<strong>{}</strong>\n时间：{}\n\n",
            gettext(&vallis.state),
            api::get_eta(&vallis.expiry)
        )
        .as_str(),
    );
    // 魔胎之境
    world_str.push_str(
        format!(
            "魔胎之境：<strong>{}</strong>\n时间：{}\n\n",
            gettext(&cambion.active),
            api::get_eta(&cambion.expiry)
        )
        .as_str(),
    );

    world_str
}
