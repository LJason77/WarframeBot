//! 裂缝

use gettextrs::gettext;

use crate::api;
use crate::models::fissures::Fissures;

pub async fn get_fissures() -> String {
    let json = api::get_url("fissures").await;
    let mut fissures_vec: Vec<Fissures> = serde_json::from_str(&json).unwrap();
    // 按照纪元排序
    fissures_vec.sort_by(|a, b| a.tier_num.cmp(&b.tier_num));

    let mut fissures_str = String::new();
    // 风暴任务
    let mut storm_str = String::new();

    for fissures in fissures_vec {
        //是否过期
        if fissures.expired {
            continue;
        }

        let s = format!(
            "<strong>{}  |  {}</strong>\n{}  |  {}\n\n",
            gettext(&fissures.tier),
            api::get_node(&fissures.node),
            gettext(&fissures.mission_type),
            api::get_eta(&fissures.expiry)
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
