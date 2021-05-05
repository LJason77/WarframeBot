use serde::{Deserialize, Serialize};

/// 仲裁
#[derive(Serialize, Deserialize)]
pub struct Arbitration {
	/// 节点
	pub node: String,
	/// 派系
	pub enemy: String,
	/// 任务类型
	#[serde(rename = "type")]
	pub mission_type: String,
	/// 结束时间
	pub expiry: String,
}
