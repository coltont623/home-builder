pub mod asset_manager;
pub mod inspector;
pub mod preferences;
pub mod settings_bar;
pub mod toolbar;

use std::collections::HashMap;
use imgui::Ui;

type UIDrawFn = fn(&Ui);

pub fn ui_registry() -> HashMap<&'static str, UIDrawFn> {
    let mut map = HashMap::new();
    map.insert("asset_manager", asset_manager::draw as UIDrawFn);
    map.insert("inspector", inspector::draw as UIDrawFn);
    map.insert("preferences", preferences::draw as UIDrawFn);
    map.insert("settings_bar", settings_bar::draw as UIDrawFn);
    map.insert("toolbar", toolbar::draw as UIDrawFn);
    map
}
