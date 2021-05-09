use serde::{Deserialize, Serialize};

/// 当前奖励
#[derive(Serialize, Deserialize)]
pub struct CurrentReward {
	/// 物品
	pub name: String,
	/// 价格
	pub cost: u8,
}

/// 钢铁之路奖励
#[derive(Serialize, Deserialize)]
pub struct SteelPath {
	/// 当前奖励
	#[serde(rename = "currentReward")]
	pub current_reward: CurrentReward,
	/// 结束时间
	pub expiry: String,
}
