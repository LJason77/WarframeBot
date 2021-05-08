//! 突击

use gettextrs::gettext;

use crate::api::{self, get_url};

pub async fn get_sortie() -> String {
	// 读取缓存
	let mut json = api::get_cache("sortie").await;
	let mut sortie: crate::models::sortie::Sortie = serde_json::from_str(&json).unwrap();

	if api::need_update(&sortie.expiry) {
		json = get_url("sortie").await;
		sortie = serde_json::from_str(&json).unwrap();
	}

	// 更新缓存
	api::update_cache(&json, "sortie");

	let mut variants = String::new();
	for (i, variant) in sortie.variants.iter().enumerate() {
		variants.push_str(
			format!(
				r#"
				{}. {}
				任务：{}
				状态：{}
				"#,
				i + 1,
				api::get_node(&variant.node),
				gettext(&variant.mission_type),
				gettext(&variant.modifier)
			)
			.as_str(),
		)
	}

	format!(
		r#"
突击阵营：{}
阵营领导者：{}
剩余时间：{}
{}
		"#,
		gettext(sortie.faction),
		gettext(sortie.boss),
		api::get_eta(&sortie.expiry),
		variants
	)
}
