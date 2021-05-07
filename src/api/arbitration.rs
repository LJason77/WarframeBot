//! 仲裁

use std::{
	fs::{self, File},
	io::Write,
	path::Path,
};

use gettextrs::gettext;

use crate::api::{self, get_url};

pub async fn get_arbitration() -> String {
	// 读取缓存
	let mut json = match fs::read_to_string("cache/arbitration.json") {
		Ok(json) => json,
		Err(_) => get_url("arbitration").await,
	};
	let mut arbitration: crate::models::arbitration::Arbitration = serde_json::from_str(&json).unwrap();

	if api::need_update(&arbitration.expiry) {
		json = get_url("arbitration").await;
		arbitration = serde_json::from_str(&json).unwrap();
	}

	// 更新缓存
	let json_file = Path::new("cache/arbitration.json");
	let mut file = match File::create(&json_file) {
		Err(error) => panic!("无法创建 {}：{}", json_file.display(), error),
		Ok(file) => file,
	};
	if let Err(why) = file.write_all(&json.as_bytes()) {
		println!("更新缓存失败：{}", why)
	}

	format!(
		"地点：{}\n派系：{}\n任务：{}\n剩余：{}",
		api::get_node(&arbitration.node),
		arbitration.enemy,
		gettext(arbitration.mission_type),
		api::get_eta(&arbitration.expiry)
	)
}
