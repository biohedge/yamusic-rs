//! Яндекс Музыка RS — неофициальный клиент Яндекс Музыки. Точка входа: один экземпляр
//! на пользователя, выбор рендерера, запуск окна.

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod ahead;
mod api;
mod assets;
mod auth;
mod client;
mod config;
mod discord;
mod eq;
mod fade;
mod flac;
mod gui;
mod history;
mod i18n;
mod images;
mod landing;
#[cfg(target_os = "macos")]
mod mac;
#[cfg(target_os = "linux")]
mod mpris;
mod net;
mod output;
mod palette;
mod player;
mod save;
mod secret;
#[cfg(windows)]
mod smtc;
mod stream;
#[cfg_attr(windows, path = "sys/windows.rs")]
#[cfg_attr(target_os = "linux", path = "sys/linux.rs")]
#[cfg_attr(target_os = "macos", path = "sys/macos.rs")]
mod sys;
mod tags;
#[cfg(windows)]
mod tray;
mod update;
mod yandex_id;
mod ynison;

fn main() -> Result<(), slint::PlatformError> {
    // В release stderr теряется: панику пишем в лог (сообщение + место),
    // чтобы краши можно было разбирать по log.txt без crash report'ов.
    std::panic::set_hook(Box::new(|info| {
        config::log(format!("PANIC: {info}"));
    }));
    #[cfg(windows)]
    smtc::register_app_id();
    #[cfg(target_os = "linux")]
    sys::audio_identity();
    // После обновления старая версия ещё закрывается: ждём её до 15 секунд.
    let after_update = std::env::args().any(|a| a == update::AFTER_UPDATE);
    let mut claimed = sys::claim_instance();
    for _ in 0..60 {
        if claimed || !after_update {
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(250));
        claimed = sys::claim_instance();
    }
    if !claimed {
        // Второй запуск просто показывает уже открытое окно.
        sys::wake_existing();
        return Ok(());
    }
    // Дефолт — CPU (skia-software): так задумал автор (память и совместимость
    // не зависят от драйвера). GPU (Skia через wgpu: macOS — Metal, Windows —
    // DX12/Vulkan) включается в настройках; замер на macOS дал CPU 25.1% → 4.3%,
    // RSS 178 → 137 МБ, но на слабых машинах решение остаётся за пользователем.
    // Env YAMUSIC_RENDERER перекрывает конфиг; при ошибке выбора GPU — fallback.
    // Режим работы («Плавный» или «Лёгкий») меняет только анимации и кэши
    // и переключается на лету (gui::apply_mode).
    let want = std::env::var("YAMUSIC_RENDERER").unwrap_or_default();
    let gpu_on = config::Config::load().gpu;
    let renderer = match want.as_str() {
        "skia-software" | "skia" | "skia-opengl" | "skia-wgpu" => want.clone(),
        _ => if gpu_on { "skia-wgpu".to_string() } else { "skia-software".to_string() },
    };
    config::log(format!("renderer: {renderer}"));
    let selected = slint::BackendSelector::new().backend_name("winit".into()).renderer_name(renderer.clone()).select();
    if selected.is_err() && renderer != "skia-software" {
        config::log(format!("renderer {renderer} недоступен, fallback на skia-software"));
        slint::BackendSelector::new().backend_name("winit".into()).renderer_name("skia-software".into()).select()?;
    } else {
        selected?;
    }
    // По app_id (Wayland) и WM_CLASS (X11) окружение находит .desktop и значок.
    #[cfg(target_os = "linux")]
    slint::set_xdg_app_id("yamusic-rs")?;
    update::cleanup();
    sys::refresh_autostart();
    // Без трея на Linux и macOS спрятанное окно было бы не вернуть из --tray.
    let start_hidden = cfg!(windows) && std::env::args().any(|a| a == "--tray");
    gui::run(start_hidden)
}
