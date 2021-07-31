//! 世界状态

use gettextrs::gettext;

use crate::models::worldstate::{CambionCycle, EarthCycle, VallisCycle};

use super::{get_cache, get_eta, get_url, need_update};

pub async fn get_world() -> String {
    // 读取缓存
    let mut earth = match get_cache::<EarthCycle>("earthCycle").await {
        Ok(earth) => earth,
        Err(err) => return err,
    };

    let mut cetus = match get_cache::<EarthCycle>("cetusCycle").await {
        Ok(cetus) => cetus,
        Err(err) => return err,
    };

    let mut vallis = match get_cache::<VallisCycle>("vallisCycle").await {
        Ok(vallis) => vallis,
        Err(err) => return err,
    };

    let mut cambion = match get_cache::<CambionCycle>("cambionCycle").await {
        Ok(cambion) => cambion,
        Err(err) => return err,
    };

    if need_update(&earth.expiry) {
        earth = match get_url::<EarthCycle>("earthCycle", None).await {
            Ok(earth) => earth,
            Err(err) => return err,
        };
    }
    if need_update(&cetus.expiry) {
        cetus = match get_url::<EarthCycle>("cetusCycle", None).await {
            Ok(cetus) => cetus,
            Err(err) => return err,
        };
    }
    if need_update(&vallis.expiry) {
        vallis = match get_url::<VallisCycle>("vallisCycle", None).await {
            Ok(vallis) => vallis,
            Err(err) => return err,
        };
    }

    if need_update(&cambion.expiry) {
        cambion = match get_url::<CambionCycle>("cambionCycle", None).await {
            Ok(cambion) => cambion,
            Err(err) => return err,
        };
    }

    let mut world_str = String::new();

    // 地球昼夜
    world_str.push_str(
        format!(
            "地球：<strong>{}</strong>\n时间：{}\n\n",
            gettext(&earth.state),
            get_eta(&earth.expiry)
        )
        .as_str(),
    );
    // 希图斯昼夜
    world_str.push_str(
        format!(
            "希图斯：<strong>{}</strong>\n时间：{}\n\n",
            gettext(&cetus.state),
            get_eta(&cetus.expiry)
        )
        .as_str(),
    );
    // 奥布山谷
    world_str.push_str(
        format!(
            "奥布山谷：<strong>{}</strong>\n时间：{}\n\n",
            gettext(&vallis.state),
            get_eta(&vallis.expiry)
        )
        .as_str(),
    );
    // 魔胎之境
    world_str.push_str(
        format!(
            "魔胎之境：<strong>{}</strong>\n时间：{}\n\n",
            gettext(&cambion.active),
            get_eta(&cambion.expiry)
        )
        .as_str(),
    );

    world_str
}
