//! Системный плеер Windows (SMTC): карточка с треком и обложкой в панели Windows,
//! медиаклавиши клавиатуры и кнопки Bluetooth-наушников.

use crate::api::Track;
use crate::config::log;
use crate::player::{Player, Shared};
use std::path::Path;
use std::ptr::{null, null_mut};
use std::sync::Arc;
use std::time::{Duration, Instant};
use windows::core::{factory, Interface, GUID, HSTRING, PCWSTR};
use windows::Foundation::{TimeSpan, TypedEventHandler, Uri};
use windows::Media::{
    MediaPlaybackStatus, MediaPlaybackType, PlaybackPositionChangeRequestedEventArgs,
    SystemMediaTransportControls, SystemMediaTransportControlsButton,
    SystemMediaTransportControlsButtonPressedEventArgs,
    SystemMediaTransportControlsTimelineProperties,
};
use windows::Storage::Streams::RandomAccessStreamReference;
use windows::Win32::Foundation::{HWND, PROPERTYKEY};
use windows::Win32::System::Com::StructuredStorage::PROPVARIANT;
use windows::Win32::System::Com::{
    CoCreateInstance, CoInitializeEx, IPersistFile, CLSCTX_INPROC_SERVER, COINIT_MULTITHREADED,
    STGM_READ,
};
use windows::Win32::System::WinRT::ISystemMediaTransportControlsInterop;
use windows::Win32::UI::Shell::PropertiesSystem::IPropertyStore;
use windows::Win32::UI::Shell::{IShellLinkW, ShellLink};
use windows_sys::Win32::System::LibraryLoader::GetModuleHandleW;
use windows_sys::Win32::UI::Shell::SetCurrentProcessExplicitAppUserModelID;
use windows_sys::Win32::UI::WindowsAndMessaging::{
    CreateWindowExW, DefWindowProcW, DispatchMessageW, PeekMessageW, RegisterClassW,
    TranslateMessage, MSG, PM_REMOVE, WNDCLASSW,
};

const AUMID: &str = "SpolzerMod.YaMusicRS";

/// `System.AppUserModel.ID` (`{9F4C2855-9F79-4B39-A8D0-E1D42DE1D5F3}`, pid 5).
const PKEY_APP_USER_MODEL_ID: PROPERTYKEY = PROPERTYKEY {
    fmtid: GUID::from_u128(0x9F4C2855_9F79_4B39_A8D0_E1D42DE1D5F3),
    pid: 5,
};

/// Задаёт AppUserModelID процесса, чтобы Windows 11 показывала имя и значок приложения
/// в карточке системного плеера вместо «Unknown app». Вызывать в начале `main()` до создания окон.
pub fn register_app_id() {
    let aumid = crate::sys::wide(AUMID);
    unsafe {
        let _ = SetCurrentProcessExplicitAppUserModelID(aumid.as_ptr());
    }
}

/// Команда от системного плеера Windows. Время — в секундах.
pub enum Event {
    Play,
    Pause,
    Next,
    Prev,
    Seek(f32),
}

/// Запустить интеграцию с Windows SMTC в отдельном потоке.
pub fn spawn(player: Player, on_event: impl Fn(Event) + Send + Sync + 'static) {
    let started = std::thread::Builder::new().name("smtc".into()).spawn(move || {
        if let Err(e) = run(player, Arc::new(on_event)) {
            log(format!("SMTC: {e}"));
        }
    });
    if let Err(e) = started {
        log(format!("SMTC: поток не запустился: {e}"));
    }
}

/// Создаёт или обновляет ярлык в меню «Пуск» с нашим `AppUserModelID`: по нему Windows 11
/// подставляет имя и значок приложения в карточку системного плеера. Если ярлык уже указывает
/// на этот же exe с нужным `AppUserModelID`, файл не трогаем.
fn ensure_start_menu_shortcut() -> windows::core::Result<()> {
    let Ok(exe) = std::env::current_exe() else {
        return Ok(());
    };
    let programs = crate::sys::config_base().join(r"Microsoft\Windows\Start Menu\Programs");
    if !programs.is_dir() {
        return Ok(());
    }
    let lnk = programs.join(format!("{}.lnk", env!("APP_NAME")));
    if lnk.exists() && shortcut_matches(&lnk, &exe) {
        return Ok(());
    }
    write_shortcut(&lnk, &exe)
}

fn shortcut_matches(lnk: &Path, exe: &Path) -> bool {
    let lnk_w = HSTRING::from(lnk.as_os_str());
    unsafe {
        let Ok(link) = CoCreateInstance::<_, IShellLinkW>(&ShellLink, None, CLSCTX_INPROC_SERVER)
        else {
            return false;
        };
        let Ok(file) = link.cast::<IPersistFile>() else {
            return false;
        };
        if file.Load(PCWSTR(lnk_w.as_ptr()), STGM_READ).is_err() {
            return false;
        }
        let mut buf = [0u16; 1024];
        if link.GetPath(&mut buf, null_mut(), 0).is_err() {
            return false;
        }
        let n = buf.iter().position(|&c| c == 0).unwrap_or(buf.len());
        let target = String::from_utf16_lossy(&buf[..n]);
        if target.to_lowercase() != exe.to_string_lossy().to_lowercase() {
            return false;
        }
        let Ok(store) = link.cast::<IPropertyStore>() else {
            return false;
        };
        let Ok(val) = store.GetValue(&PKEY_APP_USER_MODEL_ID) else {
            return false;
        };
        val.to_string() == AUMID
    }
}

fn write_shortcut(lnk: &Path, exe: &Path) -> windows::core::Result<()> {
    let exe_w = HSTRING::from(exe.as_os_str());
    let lnk_w = HSTRING::from(lnk.as_os_str());
    unsafe {
        let link: IShellLinkW = CoCreateInstance(&ShellLink, None, CLSCTX_INPROC_SERVER)?;
        link.SetPath(PCWSTR(exe_w.as_ptr()))?;
        link.SetIconLocation(PCWSTR(exe_w.as_ptr()), 0)?;
        let store: IPropertyStore = link.cast()?;
        let val = PROPVARIANT::from(AUMID);
        store.SetValue(&PKEY_APP_USER_MODEL_ID, &val)?;
        store.Commit()?;
        let file: IPersistFile = link.cast()?;
        file.Save(PCWSTR(lnk_w.as_ptr()), true)?;
    }
    Ok(())
}

fn create_hidden_window() -> Option<HWND> {
    unsafe {
        let inst = GetModuleHandleW(null());
        let class = crate::sys::wide("yamusic_rs_smtc");
        let title = crate::sys::wide(env!("APP_NAME"));
        let mut wc: WNDCLASSW = std::mem::zeroed();
        wc.lpfnWndProc = Some(DefWindowProcW);
        wc.hInstance = inst;
        wc.lpszClassName = class.as_ptr();
        RegisterClassW(&wc);
        let hwnd = CreateWindowExW(
            0,
            class.as_ptr(),
            title.as_ptr(),
            0,
            0,
            0,
            0,
            0,
            null_mut(),
            null_mut(),
            inst,
            null(),
        );
        if hwnd.is_null() {
            None
        } else {
            Some(HWND(hwnd as *mut _))
        }
    }
}

fn pump_messages() {
    unsafe {
        let mut msg: MSG = std::mem::zeroed();
        while PeekMessageW(&mut msg, null_mut(), 0, 0, PM_REMOVE) != 0 {
            TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
    }
}

fn run(
    player: Player,
    on_event: Arc<dyn Fn(Event) + Send + Sync>,
) -> windows::core::Result<()> {
    unsafe {
        let _ = CoInitializeEx(None, COINIT_MULTITHREADED);
    }
    if let Err(e) = ensure_start_menu_shortcut() {
        log(format!("SMTC: не удалось обновить ярлык в меню Пуск: {e}"));
    }
    let Some(hwnd) = create_hidden_window() else {
        log("SMTC: не удалось создать служебное окно");
        return Ok(());
    };

    let interop: ISystemMediaTransportControlsInterop =
        factory::<SystemMediaTransportControls, ISystemMediaTransportControlsInterop>()?;
    let smtc: SystemMediaTransportControls = unsafe { interop.GetForWindow(hwnd)? };

    smtc.SetIsEnabled(true)?;
    smtc.SetIsPlayEnabled(true)?;
    smtc.SetIsPauseEnabled(true)?;
    smtc.SetIsStopEnabled(true)?;
    smtc.SetIsNextEnabled(false)?;
    smtc.SetIsPreviousEnabled(false)?;
    smtc.SetPlaybackStatus(MediaPlaybackStatus::Closed)?;

    let cb = on_event.clone();
    let _btn_token = smtc.ButtonPressed(&TypedEventHandler::<
        SystemMediaTransportControls,
        SystemMediaTransportControlsButtonPressedEventArgs,
    >::new(move |_, args| {
        if let Some(args) = args.as_ref() {
            if let Ok(btn) = args.Button() {
                match btn {
                    SystemMediaTransportControlsButton::Play => cb(Event::Play),
                    SystemMediaTransportControlsButton::Pause
                    | SystemMediaTransportControlsButton::Stop => cb(Event::Pause),
                    SystemMediaTransportControlsButton::Next => cb(Event::Next),
                    SystemMediaTransportControlsButton::Previous => cb(Event::Prev),
                    _ => {}
                }
            }
        }
        Ok(())
    }))?;

    let cb = on_event;
    let _seek_token = smtc.PlaybackPositionChangeRequested(&TypedEventHandler::<
        SystemMediaTransportControls,
        PlaybackPositionChangeRequestedEventArgs,
    >::new(move |_, args| {
        if let Some(args) = args.as_ref() {
            if let Ok(pos) = args.RequestedPlaybackPosition() {
                let secs = (pos.Duration as f64 / 10_000_000.0).max(0.0) as f32;
                cb(Event::Seek(secs));
            }
        }
        Ok(())
    }))?;

    let mut last_track: Option<String> = None;
    let mut last_status: Option<MediaPlaybackStatus> = None;
    let mut last_duration: f32 = 0.0;
    let mut last_pos: Option<(Instant, f32)> = None;

    loop {
        pump_messages();
        let s = player.state();
        let id = s.track.as_ref().map(|t| t.id.clone());
        let track_changed = id != last_track;

        if track_changed {
            let has = s.track.is_some();
            let _ = smtc.SetIsNextEnabled(has);
            let _ = smtc.SetIsPreviousEnabled(has);
            update_metadata(&smtc, s.track.as_ref());
            last_track = id;
            last_duration = 0.0;
            last_pos = None;
        }

        let st = playback_status(&s);
        let status_changed = Some(st) != last_status;
        if status_changed {
            let _ = smtc.SetPlaybackStatus(st);
            last_status = Some(st);
        }

        let dur = effective_duration(&s);
        let dur_changed = (dur - last_duration).abs() > 0.5;
        let expected = last_pos
            .map(|(at, pos)| if s.playing { pos + at.elapsed().as_secs_f32() } else { pos });
        let jumped = expected.is_some_and(|e| (e - s.position).abs() > 1.5);

        if s.track.is_some() && dur > 0.0 && (track_changed || status_changed || dur_changed || jumped)
        {
            update_timeline(&smtc, s.position, dur);
            last_duration = dur;
        }
        last_pos = Some((Instant::now(), s.position));

        std::thread::sleep(Duration::from_millis(300));
    }
}

fn playback_status(s: &Shared) -> MediaPlaybackStatus {
    match (&s.track, s.playing) {
        (None, _) => MediaPlaybackStatus::Closed,
        (Some(_), true) => MediaPlaybackStatus::Playing,
        (Some(_), false) => MediaPlaybackStatus::Paused,
    }
}

fn effective_duration(s: &Shared) -> f32 {
    if s.duration > 0.0 {
        s.duration
    } else if let Some(t) = &s.track {
        if t.duration_ms > 0 {
            t.duration_ms as f32 / 1000.0
        } else {
            0.0
        }
    } else {
        0.0
    }
}

fn update_metadata(smtc: &SystemMediaTransportControls, track: Option<&Track>) {
    let Ok(updater) = smtc.DisplayUpdater() else { return };
    let _ = updater.ClearAll();
    let Some(t) = track else {
        let _ = updater.Update();
        return;
    };
    let _ = updater.SetType(MediaPlaybackType::Music);
    if let Ok(music) = updater.MusicProperties() {
        let title = if t.version.is_empty() {
            t.title.clone()
        } else {
            format!("{} ({})", t.title, t.version)
        };
        let _ = music.SetTitle(&HSTRING::from(&title));
        let _ = music.SetArtist(&HSTRING::from(&t.artists));
        if !t.album_title.is_empty() {
            let _ = music.SetAlbumTitle(&HSTRING::from(&t.album_title));
        }
    }
    if let Some(url) = t.cover_url(400) {
        if let Ok(uri) = Uri::CreateUri(&HSTRING::from(&url)) {
            if let Ok(thumb) = RandomAccessStreamReference::CreateFromUri(&uri) {
                let _ = updater.SetThumbnail(&thumb);
            }
        }
    }
    let _ = updater.Update();
}

fn update_timeline(smtc: &SystemMediaTransportControls, position: f32, duration: f32) {
    let Ok(tl) = SystemMediaTransportControlsTimelineProperties::new() else { return };
    let end = ticks(duration);
    let pos = ticks(position.clamp(0.0, duration));
    let _ = tl.SetStartTime(ticks(0.0));
    let _ = tl.SetMinSeekTime(ticks(0.0));
    let _ = tl.SetEndTime(end);
    let _ = tl.SetMaxSeekTime(end);
    let _ = tl.SetPosition(pos);
    let _ = smtc.UpdateTimelineProperties(&tl);
}

fn ticks(secs: f32) -> TimeSpan {
    TimeSpan { Duration: (secs.max(0.0) as f64 * 10_000_000.0) as i64 }
}
