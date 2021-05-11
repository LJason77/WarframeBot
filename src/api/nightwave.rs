//! 午夜电波

use gettextrs::gettext;

use crate::api::{self, get_eta};

pub async fn get_nightwave() -> String {
	// 读取缓存
	let (mut json, mut nightwave) = api::get_cache::<crate::models::nightwave::Nightwave>("nightwave").await;

	for challenge in &nightwave.challenges.to_owned() {
		if api::need_update(&challenge.expiry) {
			json = api::get_url("nightwave").await;
			nightwave = serde_json::from_str(&json).unwrap();
		}
	}

	// 更新缓存
	api::update_cache(&json, "nightwave");

	let mut challenges = String::new();
	for challenge in nightwave.challenges {
		challenges.push_str(
			format!(
				r#"
					<strong>{}  |  {}</strong>
					声望：{}  |  剩余：{}
					"#,
				gettext(&challenge.title),
				gettext(&challenge.desc),
				&challenge.reputation,
				get_eta(&challenge.expiry)
			)
			.as_str(),
		)
	}

	format!("电波剩余时间：{}\n  {}", get_eta(&nightwave.expiry), challenges)
}
