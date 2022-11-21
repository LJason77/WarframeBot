//! 执刑官猎杀

use gettextrs::gettext;

use crate::models::Archon;

use super::{get_cache, get_eta, get_node, get_url, need_update};

pub async fn get_archon() -> String {
    // 读取缓存
    let mut archon = match get_cache::<Archon>("archonHunt").await {
        Ok(archon) => archon,
        Err(err) => return err,
    };

    if need_update(&archon.expiry) {
        archon = match get_url("archonHunt", None).await {
            Ok(archon) => archon,
            Err(err) => return err,
        };
    }

    let mut missions = String::new();
    for (i, mission) in archon.missions.iter().enumerate() {
        missions.push_str(
            format!(
                r#"
				{}. {}
				任务：<strong>{}</strong>
				"#,
                i + 1,
                get_node(&mission.node),
                gettext(&mission.mission_type),
            )
            .as_str(),
        );
    }

    format!(
        r#"
领导者：{}
剩余时间：{}
{}
		"#,
        gettext(archon.boss),
        get_eta(&archon.expiry),
        missions
    )
}
