use serde::{Deserialize, Serialize};

/// 物品
#[derive(Debug, Serialize, Deserialize)]
pub struct Item {
	/// 物品
	pub item: String,
	/// 杜卡德金币
	pub ducats: i16,
	/// 现金
	pub credits: i32,
}

/// 虚空商人
#[derive(Debug, Serialize, Deserialize)]
pub struct Trader {
	/// 抵达时间
	pub activation: String,
	/// 结束时间
	pub expiry: String,
	/// 地点
	pub location: String,
	/// 状态
	pub active: bool,
	/// 清单
	pub inventory: Vec<Item>,
}
