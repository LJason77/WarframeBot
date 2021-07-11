use std::io::Write;

use chrono::Duration;
use gettextrs::gettext;
use serde::de::DeserializeOwned;
use serde_json::from_str;

pub mod arbitration;
pub mod event;
pub mod fissures;
pub mod invasion;
pub mod new;
pub mod nightwave;
pub mod sortie;
pub mod steel_path;
pub mod syndicate;
pub mod trader;
pub mod worldstate;

/// 发送请求
pub async fn get_url(path: &str) -> String {
    reqwest::Client::builder()
        // 将所有流量代理到传递的URL
        .proxy(reqwest::Proxy::all(std::env::var("TELOXIDE_PROXY").unwrap()).unwrap())
        .build()
        .unwrap()
        .get(format!("https://api.warframestat.us/pc/{}", path))
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap()
}

/// 读取缓存
pub async fn get_cache<T>(path: &str) -> (String, T)
where
    T: DeserializeOwned,
{
    let json = match std::fs::read_to_string(format!("cache/{}.json", path)) {
        Ok(json) => json,
        Err(_) => get_url(path).await,
    };
    let t: T = from_str(&json).unwrap();
    (json, t)
}

/// 更新缓存
pub fn update_cache(json: &str, path: &str) {
    let path = format!("cache/{}.json", path);
    let json_file = std::path::Path::new(&path);
    let mut file = match std::fs::File::create(json_file) {
        Err(error) => panic!("无法创建 {}：{}", json_file.display(), error),
        Ok(file) => file,
    };
    if let Err(why) = file.write_all(json.as_bytes()) {
        println!("更新缓存失败：{}", why)
    }
}

/// 翻译星球名字
pub fn get_node(node: &str) -> String {
    match node.find('(') {
        Some(l) => {
            let r = node.find(')').unwrap();
            format!("{} ({})", gettext(&node[..l - 1]), gettext(&node[l + 1..r]))
        }
        None => gettext(node),
    }
}

/// 计算剩余时间
pub fn get_eta(expiry: &str) -> String {
    let expiry = chrono::DateTime::parse_from_rfc3339(expiry)
        .unwrap()
        .naive_utc();
    let local_time = chrono::Utc::now().naive_utc();
    let mut duration = expiry - local_time;

    let mut eta = String::new();
    let days = duration.num_days();
    if days > 0 {
        eta.push_str(format!("{} 天 ", days).as_str());
        duration = duration - Duration::days(days);
    }

    let hours = duration.num_hours();
    eta.push_str(format!("{} 时 ", hours).as_str());
    duration = duration - Duration::hours(hours);

    let minutes = duration.num_minutes();
    eta.push_str(format!("{} 分 ", minutes).as_str());
    duration = duration - Duration::minutes(minutes);

    eta.push_str(format!("{} 秒", duration.num_seconds()).as_str());

    eta
}

/// 是否需要更新
pub fn need_update(expiry: &str) -> bool {
    // 现在时间
    let now = chrono::Utc::now().naive_utc();
    // 结束时间
    let expiry = chrono::DateTime::parse_from_rfc3339(expiry)
        .unwrap()
        .naive_utc();
    let duration = expiry - now;
    // 如果小于 0，即缓存过时
    duration.lt(&Duration::zero())
}
