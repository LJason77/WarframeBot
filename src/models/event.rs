use serde::{Deserialize, Serialize};

/// 活动
#[derive(Serialize, Deserialize)]
pub struct Event {
	/// 标题
	pub description: String,
	/// 结束时间
	pub expiry: String,
}
