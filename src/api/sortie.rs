//! 突击

use std::{
	fs::{self, File},
	io::Write,
	path::Path,
};

use gettextrs::gettext;

pub async fn get_sortie() -> String {
	// 读取缓存
	let mut json = match fs::read_to_string("cache/sortie.json") {
		Ok(json) => json,
		Err(_) => crate::api::get_url("https://api.warframestat.us/pc/sortie").await,
	};
	let mut sortie: crate::models::sortie::Sortie = serde_json::from_str(&json).unwrap();

	// 现在时间
	let now = chrono::Utc::now().naive_utc();
	// 结束时间
	let expiry = chrono::DateTime::parse_from_rfc3339(&sortie.expiry)
		.unwrap()
		.naive_utc();
	let duration = expiry - now;
	// 如果小于 0，即缓存过时
	if duration.lt(&chrono::Duration::zero()) {
		json = crate::api::get_url("https://api.warframestat.us/pc/sortie").await;
		sortie = serde_json::from_str(&json).unwrap();
	}

	// 更新缓存
	let sortie_file = Path::new("cache/sortie.json");
	let mut file = match File::create(&sortie_file) {
		Err(error) => panic!("无法创建 {}：{}", sortie_file.display(), error),
		Ok(file) => file,
	};
	if let Err(why) = file.write_all(&json.as_bytes()) {
		println!("更新缓存失败：{}", why)
	}

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
		crate::api::get_eta(&sortie.expiry),
		variants
	)
}
