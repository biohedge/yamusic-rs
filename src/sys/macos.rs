//! macOS: окно через winit, пути в ~/Library, диалоги osascript, автозапуск LaunchAgent.
//! Трея нет: закрытие при играющем треке прячет окно, приложение остаётся в Dock.
//! Второй экземпляр .app не стартует — его показывает Launch Services.

use slint::winit_030::WinitWindowAccessor;
use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Stdio};

pub fn created(w: &slint::Window) -> bool {
    w.with_winit_window(|_| ()).is_some()
}

/// Светофор и скругление. Значок берётся из .app (tools/make_app.sh).
pub fn prepare(w: &slint::Window) {
    let view = ns_view(w);
    if !view.is_null() {
        crate::mac::native_window_buttons(view);
    }
}

/// Wayland-ограничений нет: окно просто выводим вперёд.
pub fn raise(w: &slint::Window) {
    w.set_minimized(false);
    w.with_winit_window(|w| w.focus_window());
}

pub fn minimize(w: &slint::Window) {
    w.set_minimized(true);
}

pub fn scale(w: &slint::Window) -> f32 {
    w.with_winit_window(|w| w.scale_factor() as f32).unwrap_or(1.0)
}

/// Рабочая область основного экрана в физических пикселях (лево, верх, право, низ).
pub fn work_area(_w: &slint::Window) -> (i32, i32, i32, i32) {
    crate::mac::work_area()
}

/// Системный диалог выбора картинки (JPEG, PNG). `None` — пользователь отказался.
/// `osascript` печатает POSIX-путь без кавычек.
pub fn pick_image(_: &slint::Window, title: &str) -> Option<String> {
    let t = title.replace('\\', "\\\\").replace('"', "\\\"");
    let script = format!("POSIX path of (choose file with prompt \"{t}\" of type {{\"public.jpeg\", \"public.png\"}})");
    let out = Command::new("osascript").arg("-e").arg(script).output().ok()?;
    if !out.status.success() {
        return None;
    }
    Some(String::from_utf8_lossy(&out.stdout).trim().to_string())
}

/// AppleScript диалога выбора папки. `default location` подставляется, только если
/// папка существует: AppleScript коэрсит путь в `alias` и падает на несуществующем
/// (`-1700 Can't make file … into type alias`), из-за чего диалог не открывается.
fn choose_folder_script(title: &str, start: &str) -> String {
    let esc = |s: &str| s.replace('\\', "\\\\").replace('"', "\\\"");
    let def = if !start.is_empty() && std::path::Path::new(start).exists() {
        format!(" default location (POSIX file \"{}\")", esc(start))
    } else {
        String::new()
    };
    format!("POSIX path of (choose folder with prompt \"{}\"{def})", esc(title))
}

/// Системный диалог выбора папки (`choose folder` через osascript).
pub fn pick_folder(_: &slint::Window, title: &str, start: &str) -> Option<String> {
    let script = choose_folder_script(title, start);
    let out = Command::new("osascript").arg("-e").arg(script).output().ok()?;
    if !out.status.success() {
        return None;
    }
    let s = String::from_utf8_lossy(&out.stdout).trim().to_string();
    // osascript возвращает путь с завершающим «/» — уберём, кроме корня.
    Some(if s.len() > 1 { s.trim_end_matches('/').to_string() } else { s })
}

pub fn open_url(url: &str) {
    let _ = Command::new("open").arg(url).stdout(Stdio::null()).stderr(Stdio::null()).spawn();
}

/// Память на macOS отдаёт система сама: аналога SetProcessWorkingSetSize нет.
pub fn trim_memory() {}

pub fn release_memory() {
    trim_memory();
}

/// Светлая ли тема: `defaults` отвечает «Dark» при тёмной теме,
/// а на светлой ругается, что ключа нет.
pub fn system_light_theme() -> bool {
    Command::new("defaults").args(["read", "-g", "AppleInterfaceStyle"]).output().is_ok_and(|o| !o.status.success())
}

fn home() -> PathBuf {
    PathBuf::from(std::env::var("HOME").unwrap_or_else(|_| ".".into()))
}

pub fn config_base() -> PathBuf {
    home().join("Library").join("Application Support")
}

/// Кэш рядом с настройками: шрифты уже лежат в Application Support, перенос в
/// ~/Library/Caches заставил бы скачать их заново.
pub fn cache_base() -> PathBuf {
    config_base()
}

pub fn music_dir() -> PathBuf {
    home().join("Music")
}

pub fn hostname() -> Option<String> {
    let out = Command::new("scutil").args(["--get", "ComputerName"]).output().ok()?;
    let name = String::from_utf8(out.stdout).ok()?;
    Some(name.trim().to_string()).filter(|n| !n.is_empty())
}

pub fn copy_text(text: &str) {
    if let Ok(mut child) = Command::new("pbcopy").stdin(Stdio::piped()).spawn() {
        if let Some(mut stdin) = child.stdin.take() {
            let _ = stdin.write_all(text.as_bytes());
        }
        let _ = child.wait();
    }
}

// ---------- автозапуск: LaunchAgent ----------

const LAUNCH_LABEL: &str = "ru.spolzermod.yamusic-rs";

fn launch_agent_path() -> Option<PathBuf> {
    let p = home().join("Library").join("LaunchAgents").join(format!("{LAUNCH_LABEL}.plist"));
    std::fs::create_dir_all(p.parent()?).ok()?;
    Some(p)
}

/// Записан ли автозапуск.
pub fn autostart_enabled() -> bool {
    launch_agent_path().is_some_and(|p| p.exists())
}

/// Включить или выключить автозапуск (plist в ~/Library/LaunchAgents).
/// Бинарник запускается обычным окном: трея на macOS нет.
pub fn set_autostart(on: bool) -> bool {
    let Some(p) = launch_agent_path() else { return false };
    if !on {
        return match std::fs::remove_file(&p) {
            Ok(()) => true,
            Err(_) => !p.exists(),
        };
    }
    let Ok(exe) = std::env::current_exe() else { return false };
    let plist = format!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<!DOCTYPE plist PUBLIC \"-//Apple//DTD PLIST 1.0//EN\" \"http://www.apple.com/DTDs/PropertyList-1.0.dtd\">\n<plist version=\"1.0\"><dict><key>Label</key><string>{LAUNCH_LABEL}</string><key>ProgramArguments</key><array><string>{}</string></array><key>RunAtLoad</key><true/></dict></plist>\n",
        exe.display()
    );
    std::fs::write(&p, plist).is_ok()
}

/// Бинарник перенесли — поправить путь в автозапуске, если он включён.
pub fn refresh_autostart() {
    if autostart_enabled() {
        set_autostart(true);
    }
}

/// .app второй раз не порождает процесс: Launch Services поднимает уже открытое окно.
pub fn claim_instance() -> bool {
    true
}

pub fn wake_existing() {}

/// Слушать нечего: второй процесс на macOS не стартует.
pub fn on_wake(_f: impl Fn() + Send + 'static) {}

pub fn cleanup() {}

fn ns_view(w: &slint::Window) -> *mut std::ffi::c_void {
    use slint::winit_030::winit::raw_window_handle::{HasWindowHandle, RawWindowHandle};
    w.with_winit_window(|ww| match ww.window_handle().map(|h| h.as_raw()) {
        Ok(RawWindowHandle::AppKit(h)) => h.ns_view.as_ptr(),
        _ => std::ptr::null_mut(),
    })
    .unwrap_or(std::ptr::null_mut())
}

#[cfg(test)]
mod tests {
    use super::choose_folder_script;

    /// Несуществующая стартовая папка не должна попадать в `default location`,
    /// иначе AppleScript падает и диалог не открывается.
    #[test]
    fn missing_default_location_is_omitted() {
        let s = choose_folder_script("Папка", "/definitely/not/exists/xyz");
        assert!(!s.contains("default location"), "диалог упадёт на несуществующем пути: {s}");
    }

    /// Существующая стартовая папка должна подставляться.
    #[test]
    fn existing_default_location_is_used() {
        let dir = std::env::temp_dir();
        let s = choose_folder_script("Папка", dir.to_str().expect("temp"));
        assert!(s.contains("default location"), "стартовая папка потерялась: {s}");
    }
}
