//! Debug-dump the current theme.

fn main() {
    let theme = ui_theme::Theme::load_blocking(None, ui_theme::ShadePreference::Light).unwrap();

    println!("Theme: {:#?}", theme);
}
