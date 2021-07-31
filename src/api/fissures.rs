//! 裂缝

use gettextrs::gettext;

use crate::models::fissures::Fissures;

use super::{get_eta, get_node, get_url};

pub async fn get_fissures() -> String {
    let mut fissures = match get_url::<Vec<Fissures>>("fissures", None).await {
        Ok(fissures) => fissures,
        Err(err) => return err,
    };
    // 按照纪元排序
    fissures.sort_by(|a, b| a.tier_num.cmp(&b.tier_num));

    let mut fissures_str = String::new();
    // 风暴任务
    let mut storm_str = String::new();

    for fissures in fissures {
        //是否过期
        if fissures.expired {
            continue;
        }

        let s = format!(
            "<strong>{}  |  {}</strong>\n{}  |  {}\n\n",
            gettext(&fissures.tier),
            get_node(&fissures.node),
            gettext(&fissures.mission_type),
            get_eta(&fissures.expiry)
        );
        if fissures.storm {
            storm_str.push_str(s.as_str());
        } else {
            fissures_str.push_str(s.as_str());
        }
    }
    fissures_str.push_str("--------------\n\n");
    fissures_str.push_str(storm_str.as_str());

    fissures_str
}
