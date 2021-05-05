use serde::{Deserialize, Serialize};

/// 裂缝
#[derive(Serialize, Deserialize)]
pub struct Fissures {
	/// 节点
	pub node: String,
	/// 纪元
	pub tier: String,
	/// 级别
	#[serde(rename = "tierNum")]
	pub tier_num: i8,
	/// 结束时间
	pub expiry: String,
	/// 是否过期
	pub expired: bool,
	/// 任务类型
	#[serde(rename = "missionType")]
	pub mission_type: String,
	/// 风暴任务
	#[serde(rename = "isStorm")]
	pub storm: bool,
}
