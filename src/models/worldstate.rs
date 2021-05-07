use serde::{Deserialize, Serialize};

/// 地球昼夜
#[derive(Serialize, Deserialize)]
pub struct EarthCycle {
	/// 是否白天
	#[serde(rename = "isDay")]
	pub is_day: bool,
	/// 状态
	pub state: String,
	/// 结束时间
	pub expiry: String,
}

/// 奥布山谷
#[derive(Serialize, Deserialize)]
pub struct VallisCycle {
	/// 是否温暖
	#[serde(rename = "isWarm")]
	pub is_warm: bool,
	/// 状态
	pub state: String,
	/// 结束时间
	pub expiry: String,
}

/// 魔胎之境
#[derive(Serialize, Deserialize)]
pub struct CambionCycle {
	/// 状态
	pub active: String,
	/// 结束时间
	pub expiry: String,
}
