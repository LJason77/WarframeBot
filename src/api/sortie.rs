//! 突击

use gettextrs::gettext;

pub async fn get_sortie() -> String {
	let json = crate::api::get_url("https://api.warframestat.us/pc/sortie").await;
	let sortie: crate::models::sortie::Sortie = serde_json::from_str(&json).unwrap();

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
				crate::api::get_node(&variant.node),
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
		sortie.eta,
		variants
	)
}
