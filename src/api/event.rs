//! 活动

use gettextrs::gettext;

use crate::api;

pub async fn get_event() -> String {
	let json = api::get_url("events").await;
	let events: Vec<crate::models::event::Event> = serde_json::from_str(&json).unwrap();

	let mut events_str = String::new();
	for event in events {
		events_str.push_str(
			format!(
				"{}\n剩余时间：{}\n\n",
				gettext(event.description),
				api::get_eta(&event.expiry)
			)
			.as_str(),
		);
	}

	events_str
}
