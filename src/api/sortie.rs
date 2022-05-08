//! 突击

use gettextrs::gettext;

use crate::models::Sortie;

use super::{get_cache, get_eta, get_node, get_url, need_update};

pub async fn get_sortie() -> String {
    // 读取缓存
    let mut sortie = match get_cache::<Sortie>("sortie").await {
        Ok(sortie) => sortie,
        Err(err) => return err,
    };

    if need_update(&sortie.expiry) {
        sortie = match get_url("sortie", None).await {
            Ok(sortie) => sortie,
            Err(err) => return err,
        };
    }

    let mut variants = String::new();
    for (i, variant) in sortie.variants.iter().enumerate() {
        variants.push_str(
            format!(
                r#"
				{}. {}
				任务：<strong>{}</strong>
				状态：<strong>{}</strong>
				"#,
                i + 1,
                get_node(&variant.node),
                gettext(&variant.mission_type),
                gettext(&variant.modifier)
            )
            .as_str(),
        );
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
        get_eta(&sortie.expiry),
        variants
    )
}
