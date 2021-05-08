//! 午夜电波

use gettextrs::gettext;

use crate::api::{self, get_eta};

pub async fn get_nightwave() -> String {
	let json = api::get_url("nightwave").await;
	let nightwave: crate::models::nightwave::Nightwave = serde_json::from_str(&json).unwrap();

	let mut challenges = String::new();
	for challenge in nightwave.challenges {
		challenges.push_str(
			format!(
				r#"
					<strong>{}</strong>
					<strong>{}</strong>
					声望：{}
					剩余时间：{}
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
