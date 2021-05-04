//! 突击

use std::{
	fs::{self, File},
	io::Write,
	path::Path,
};

use gettextrs::gettext;

use crate::api::{self, get_url};

pub async fn get_sortie() -> String {
	// 读取缓存
	let mut json = match fs::read_to_string("cache/sortie.json") {
		Ok(json) => json,
		Err(_) => get_url("https://api.warframestat.us/pc/sortie").await,
	};
	let mut sortie: crate::models::sortie::Sortie = serde_json::from_str(&json).unwrap();

	if api::need_update(&sortie.expiry) {
		json = get_url("https://api.warframestat.us/pc/sortie").await;
		sortie = serde_json::from_str(&json).unwrap();
	}

	// 更新缓存
	let json_file = Path::new("cache/sortie.json");
	let mut file = match File::create(&json_file) {
		Err(error) => panic!("无法创建 {}：{}", json_file.display(), error),
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
