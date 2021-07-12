//! 活动

use gettextrs::gettext;

use crate::{
    api::{get_eta, get_url},
    models::event::Event,
};

pub async fn get_event() -> String {
    let json = get_url("events").await;
    let events: Vec<Event> = serde_json::from_str(&json).unwrap();

    if events.is_empty() {
        return "暂无活动".to_string();
    }

    let mut events_str = String::new();
    for event in events {
        // 跳过过期活动
        if event.expired {
            continue;
        }
        events_str.push_str(
            format!(
                "{}\n剩余时间：{}\n\n",
                gettext(event.description),
                get_eta(&event.expiry)
            )
            .as_str(),
        );
    }

    events_str
}
