use warpui::platform::menu::{CustomMenuItem, MenuItem, MenuItemPropertyChanges};
use warp_core_base::features::{FeatureFlag, RUNTIME_FEATURE_FLAGS};

fn feature_flag_menu_item(flag: FeatureFlag) -> MenuItem {
    MenuItem::Custom(CustomMenuItem::new(
        &format!("{flag:?}"),
        move |_| {
            flag.set_enabled(!flag.is_enabled())
        },
        move |_props, _ctx| MenuItemPropertyChanges {
            checked: Some(flag.is_enabled()),
            ..Default::default()
        },
        None,
    ))
}

pub fn runtime_flags_menu_items() -> Vec<MenuItem> {
    if !FeatureFlag::RuntimeFeatureFlags.is_enabled() {
        return Vec::new();
    }

    RUNTIME_FEATURE_FLAGS
        .iter()
        .map(|flag| feature_flag_menu_item(*flag))
        .collect()
}
