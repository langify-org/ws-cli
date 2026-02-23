pub mod cli;
pub mod commands;
pub mod config;
pub mod context;
pub mod git;
pub mod store;
pub mod ui;

rust_i18n::i18n!("../../locales", fallback = "en");

pub fn detect_and_set_locale() {
    let locale = std::env::var("LC_ALL")
        .or_else(|_| std::env::var("LC_MESSAGES"))
        .or_else(|_| std::env::var("LANG"))
        .ok()
        .and_then(|v| {
            let v = v.trim().to_string();
            if v.is_empty() || v == "C" || v == "POSIX" {
                None
            } else {
                Some(v)
            }
        })
        .unwrap_or_else(|| sys_locale::get_locale().unwrap_or_else(|| "en".to_string()));

    let normalized = if locale.starts_with("ja") {
        "ja"
    } else if locale.starts_with("zh") {
        "zh-CN"
    } else {
        "en"
    };
    rust_i18n::set_locale(normalized);
}
