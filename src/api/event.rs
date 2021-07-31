//! 活动

use gettextrs::gettext;

use crate::models::event::Event;

use super::{get_eta, get_url};

pub async fn get_event() -> String {
    let events = match get_url::<Vec<Event>>("events", None).await {
        Ok(events) => events,
        Err(err) => return err,
    };

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
