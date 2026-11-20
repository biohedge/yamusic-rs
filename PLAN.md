# План: сборка и запуск Яндекс Музыки RS на macOS

Статус: в работе. Ключевая идея — не переписывать приложение, а добавить платформенный
слой: Win32-код изолируется в cfg-модули, для macOS пишутся свои реализации или
заглушки с теми же сигнатурами, чтобы `gui.rs` почти не менялся.

## Что уже проверено

- На маке стоит rustc 1.96 (требуется 1.92+) и Xcode CLT — всё нужное для сборки есть.
- `cargo check` на macOS: **все зависимости собираются** (slint 1.18 + winit, skia-safe
  0.153 с готовым бинарником под `aarch64-apple-darwin`, rodio/cpal → CoreAudio,
  reqwest + rustls). Ошибок компиляции в коде приложения всего две, и обе в одной
  функции `copy_text` (`src/gui.rs:1922`) — Windows-утилита `clip` и
  `std::os::windows::process::CommandExt`.
- Часть кроссплатформенности автор уже заложил: `secret.rs` и `config.rs::save()`
  имеют ветки `#[cfg(not(windows))]`, `build.rs` гейтит ресурсы exe. README при этом
  заявляет «macOS — пока не планируется», так что это локальный форк.

## Чего не хватает

1. **Линковка.** `cargo check` не линкует, а `cargo build` упадёт: `main.rs`, `win.rs`,
   `tray.rs` вызывают Win32-API без cfg-гейтов (`CreateMutexW`, `ShowWindow`,
   `Shell_NotifyIconW`, DPAPI и т.д.). `windows-sys` при этом прописан в Cargo.toml
   безусловно.
2. **Windows-допущения в остальном коде:**

   | Место | Проблема | Решение на macOS |
   | --- | --- | --- |
   | `config.rs:71` `dir()` | `%APPDATA%` не существует → пишет в CWD | `~/Library/Application Support/yamusic-rs` |
   | `gui.rs:3426` `music_dir()` | `USERPROFILE\Music` | `$HOME/Music` |
   | `gui.rs:1920` `copy_text()` | `clip.exe` + creation_flags | `pbcopy` |
   | `gui.rs:1066` `apply_ui_scale` | `win::dpi_scale(HWND)` | `ui.window().scale_factor()` (Retina = 2.0) |
   | `gui.rs:1682` `on_close` | прятать окно через `win::hide(HWND)` | `ui.hide()` (slint), приложение остаётся в Dock |
   | `main.rs` | single-instance через мьютекс Win32 | пропустить (Dock не запускает вторую копию .app); `--tray` игнорировать, пока нет трея |
   | `discord.rs:45` | пайп `\\.\pipe\discord-ipc-N` | Unix-сокет `$TMPDIR/discord-ipc-N` (протокол тот же) |
   | `update.rs` | самообновление качает `yamusic-rs.exe` | на macOS считать, что exe в релизе нет → кнопка «Обновить» ведёт на страницу релиза |
   | `assets.rs:18` | ищет `app.asar` установленного Windows-приложения | искать в `/Applications/Яндекс Музыка.app/Contents/Resources/`; сработает и веб-фолбэк (скачивание с music.yandex.ru уже кроссплатформенно) |
   | `win.rs` | всё: тема из реестра, автозапуск HKCU\Run, диалог выбора файла, `open_url` | свои реализации: `defaults`, LaunchAgent, `osascript`, `open` |

3. **Упаковка.** Голый бинарник работает, но без `.app`-бандла: нет иконки в Dock,
   имя процесса в меню-баре. Нужен скрипт сборки бандла с Info.plist и `.icns`
   из `icons/app.ico`.

## Платформенный слой (главное изменение)

- `src/win.rs` → фасад: `pub use windows::*` / `pub use unix::*` по cfg.
  - `src/win/windows.rs` — текущий код win.rs без изменений.
  - `src/win/unix.rs` — реализация для macOS:
    - рабочие: `open_url` (команда `open`), `system_light_theme` (`defaults read -g
      AppleInterfaceStyle`: ошибка = светлая), `pick_image` (`osascript choose file`),
      автозапуск (plist в `~/Library/LaunchAgents/`), `refresh_autostart`;
    - заглушки (окном управляет slint/winit, HWND всегда 0): `is_visible`, `is_iconic`,
      `show`, `hide`, `force_foreground`, `is_window`, `round_corners`, `set_icon`,
      `trim_memory`, `release_memory`.
  - только для Windows остаются `wide`, `dpi_scale`, `app_icon` (тип HICON).
- `src/tray.rs` → фасад: `Event` и `MenuState` общие (с `allow(dead_code)` вне
  Windows), текущий код в `src/tray/windows.rs`, на macOS `install/remove/
  wake_existing` — заглушки.
- `Cargo.toml`: `windows-sys` перенести в `[target.'cfg(windows)'.dependencies]`.

## Что сделано

### Этап 1 — сборка и запуск (готово)
1. `Cargo.toml` → `windows-sys` в `[target.'cfg(windows)'.dependencies]`.
2. Платформенные фасады: `src/win.rs` (+ `src/win/windows.rs`, `src/win/unix.rs`),
   `src/tray.rs` (+ `src/tray/windows.rs`, заглушки для macOS).
3. `main.rs`: cfg на `claim_instance`; на macOS `--tray` не прячет окно.
4. `gui.rs`: `copy_text` (pbcopy), `dpi_base` (scale_factor, Retina=2.0),
   `finish_reveal` ждёт появления winit-окна (было: размер окна, из-за чего
   handle был null), `on_close` → `ui.hide()`, `music_dir` → HOME.
5. `config.rs::dir()` → `~/Library/Application Support/yamusic-rs`.
6. `secret.rs`: `ENTROPY` под cfg(windows) (на macOS токен открытым текстом —
   штатный не-Windows фолбэк апстрима).
7. Проверено: сборка, линковка, `clippy -D warnings`, тесты, запуск, вход по коду,
   воспроизведение через CoreAudio, скачивание шрифтов/иконок из веб-фолбэка.

### Этап 2 — готово
8. `discord.rs`: транспорт через `UnixStream` (`$TMPDIR/discord-ipc-N`).
9. `update.rs`: на не-Windows `exe = None` → кнопка ведёт на страницу релиза.
10. `assets.rs`: поиск `app.asar` в `/Applications/Яндекс Музыка.app` и т.п.
11. `tools/make_app.sh`: `.app`-бандл (Info.plist, `.icns` из app.ico, ad-hoc codesign).

### macOS-специфика (по просьбам пользователя)
12. **Медиа-клавиши** (F7/F8/F9) и команды Пункта управления — `MPRemoteCommandCenter`
    напрямую через objc runtime (`src/mac.rs`), без сторонних крейтов. Добавлены
    `Cmd::Pause` и `Cmd::Seek`. Как у оригинала (Electron → MediaSession).
13. **«Сейчас играет»** в Пункте управления — `MPNowPlayingInfoCenter`
    (название, исполнитель, альбом, длительность, позиция, скорость).
14. **Нативные кнопки окна** («светофор») — окно winit переводится в
    titled + FullSizeContentView + прозрачный тайтлбар (как `hiddenInset` в Electron).
    Виндовые кнопки в slint скрыты флагом `App.mac`, навбар сдвинут под «светофор».
15. **Скруглённые углы** окна — `_setCornerRadius:` (приватный, с проверкой
    `respondsToSelector`), т.к. borderless-окно winit не скругляется само.
16. **Возврат «светофора» после fullscreen** — наблюдатель
    `NSWindowDidExitFullScreenNotification` перестилизует окно с задержкой 0.12 с
    (чтобы успеть после winit). Проверено самотестом: после выхода из fullscreen
    окно 1280×800, углы скруглены, `AXCloseButton`/`AXZoomButton` на месте.
    Дополнительно (план `plan/2026-11-16-macos-fixes.md`, M6/M7): радиус 24pt как
    у Finder; inset кнопок — центры (26,26)/(49,26)/(72,26) pt через
    `setFrameOrigin:`; синхронная постановка в resize-observer (live resize без
    мигания углом), отложенная на becomeKey, retry до создания кнопок.
17. **Отзывчивость «Моей волны»** — `player.rs`:
    - смена станции сразу глушит старый трек и показывает новую станцию с загрузкой;
    - фидбеки старой сессии уходят параллельным потоком, а не задерживают запрос;
    - запрос новой сессии (`replace`) идёт отдельным потоком, минуя очередь фидбеков.
18. **Потоковое воспроизведение** — `src/stream.rs`: `HttpStream` (Read+Seek через
    HTTP Range). Вместо полной загрузки 13 МБ (≈1.8 с) играет после первого куска
    256 КБ (≈0.4 с). Проверено: декодирование и перемотка побайтово совпадают
    с прямым Range-запросом (тесты `stream_and_decode`, `stream_seek_correct`).
    Перемотка (rodio 0.22, `with_coarse_seek`): прыжок к цели вместо покадрового
    скана промежутка; целевой кусок предзагружается движком (`prime`, 128 КБ),
    устаревший префетч сбрасывается в `seek`; микро-фейд 10 мс на стыке
    (`src/fade.rs`) против щелчка; UI показывает цель в кадр отпускания ползунка.
    Регрессионные тесты: `coarse_seek_does_not_stream_the_gap`,
    `seek_invalidates_stale_prefetch`, `primed_chunk_is_served_without_refetch`.
19. **Оптимизация памяти** — ветка `memory-audit` (M3–M9) смержена в `macos`:
    гейтинг `cache-rendering-hint` по видимости, весовой лимит кэша страниц,
    `Arc<Vec<Track>>` для списков + `queue_ver`, бюджеты обложек, take-паттерн
    JSON, минус лишние шрифты/спрайт, стриминг обновления, SegmentHeap-манифест
    (Windows), CI macOS-бандла. План: `plan/plan-memory-optimization.md`.
    Замер RSS macOS: лендинг 532 → 193 МБ после мержа.

## Этап 3 — что осталось (по желанию)
- настоящий трей через крейт `tray-icon` (NSStatusItem);
- CI-джоба `macos-latest` в `build.yml`, universal-бинарник (lipo);
- самообновление с отдельным ассетом `yamusic-rs-macos` + заметки про Gatekeeper;
- обновить README (таблица поддержки систем);
- XDG-пути для Linux (слой unix сейчас заточен под macOS: `defaults`, LaunchAgent);
- Keychain для токена на macOS (сейчас открытым текстом, как не-Windows фолбэк апстрима);
- обложка в карточке «Сейчас играет» (MPMediaItemArtwork).

## Риски / заметки
- `skia-software` на macOS поднялся без проблем — Metal не понадобился.
- Приватный `_setCornerRadius:` и `MPRemoteCommandCenter` — стабильные API,
  но `_setCornerRadius` приватный; если пропадёт, окно просто будет с прямыми углами
  (проверка `respondsToSelector`).

## Исправления после проверки пользователем
- **Медиа-клавиши и «Сейчас играет».** Причина: у констант `MPNowPlayingInfoProperty*`
  строковое значение — полное имя (`"MPNowPlayingInfoPropertyPlaybackRate"`), а не
  короткое, как у `MPMediaItem*`. Из-за неверных ключей `elapsedPlaybackTime`/`playbackRate`
  система не считала трек играющим → виджет пустой, клавиши не маршрутизировались.
  Исправлено, проверено на реальном воспроизведении.
- **Пауза гасила фон волны.** `sync_vibe` зависел от `playing`; теперь оттенок берётся
  от цвета трека (после первого запуска `started`), пауза его не сбрасывает.
- **«Светофор» после fullscreen.** Наблюдатель `NSWindowDidExitFullScreenNotification`
  перестилизует окно с задержкой 0.12 с (после winit). Проверено самотестом.


## Ребейз на v1.0.2 (16.11.2026)
- Ветка `macos`: мой коммит перебазирован поверх `04c7574 (1.0.2)`.
- Из upstream интегрировано: глобальные медиа-клавиши Windows (`RegisterHotKey` в
  `tray/windows.rs`), выбор папки загрузок (`win::pick_folder`), рабочая область
  монитора (`win::work_area`; на macOS — `NSScreen.visibleFrame`), мини-плеер,
  пакетное добавление в плейлист, `End::Dislike`, `.cargo/config.toml` (/Brepro).
- Конфликты решены: фасады `win.rs`/`tray.rs` сохранены, новые функции upstream
  разложены по `win/windows.rs` + `win/unix.rs`; `app.slint` — mac-гейтинг кнопок
  совмещён с 4-й кнопкой мини-плеера; `music_dir(cfg)` — папка загрузок + HOME.
- Починена Windows-ошибка `copy_text` (заимствование временного `Command`) —
  тот же фикс, что нашёл процесс memory-audit.
- Заметка: кнопка мини-плеера — в виндовых caption-кнопках, которые на macOS
  скрыты; на маке мини-плеер пока недоступен (кандидат: пункт в настройках).

## Слияние с v1.1.0 (17.11.2026)

- База — апстрим `v1.1.0` (`4ec2565`). Версия форка в `Cargo.toml` — `1.1.1`.
- Платформенный фасад `win` заменён на модуль автора `src/sys`: Windows и Linux
  взяты из апстрима, macOS — `src/sys/macos.rs`. Медиа-клавиши остались в `src/mac.rs`.
- Автозапуск macOS виден в настройках («Запускать вместе с системой»), реализация —
  прежний LaunchAgent без `--tray`.

## Слияние с v1.1.1 (17.11.2026)

- База — апстрим `v1.1.1` (`1a59b50`), 5 коммитов поверх `v1.1.0`. Код mac-слоя не задет.
- Из upstream интегрировано: скрытие блоков главной (`cfg.hidden_blocks`, строка в
  настройках возвращает блоки), ссылки из ответа сервера открываются только по https
  (`client::browser_url`), декодер главной без паники на битом ответе, без
  `SingleMainWindow` в `.desktop`.
- Конфликт был только в `README.md`: оставлена mac-строка форка. Заметку апстрима
  «macOS не поддерживается, есть сторонний порт rnikitin» в README форка не переносим.
- Ветки `macos` и `macos-1.1` влиты в `main`; рабочая ветка одна — `main`.
- Проверки: `cargo clippy --release --all-targets -- -D warnings`, `cargo test --release`
  (27 passed), `cargo build --release` — зелёные на macOS.
- Релиз форка `1.1.2`: проверка обновлений, «Обновить», «О программе» и Discord-статус
  берут адрес из `repository` в `Cargo.toml` — теперь `rnikitin/yamusic-rs`. В `README.md`
  ссылки на релизы, issues и security тоже переведены на форк.

## macOS-only сборка: dmg (17.11.2026)

- Форк публикует только macOS. В CI осталась одна джоба `macos`: clippy, тесты, universal
  `.app` и `.dmg`; Windows и Linux удалены, `release` зависит только от неё.
- `tools/make_dmg.sh` упаковывает готовый `.app` через `hdiutil` (HFS+, UDZO): на томе
  приложение и симлинк `/Applications`, оформления нет.
- Релиз `1.1.3` — один файл `yamusic-rs-1.1.3.dmg`.

### Релизы собираются локально (17.11.2026)

- GitHub 1,5 часа не выдавал macOS-раннеры: job висел в `queued`, аннотация «capacity
  constraints», после 15 минут его снимали. Ничего в коде это не лечило.
- Поэтому релиз делает `tools/release.sh`: чистое дерево, `HEAD` в `fork/main`, universal
  `.app` → `.dmg`, `hdiutil verify`, тег и `gh release create` со списком изменений и SHA-256.
- CI больше не публикует релизы: осталась одна проверка `macos` (clippy, тесты, `.app`
  и `.dmg` в артефактах).

### Переименование в «Я.Музыка [RS]» (17.11.2026)

- Отображаемое имя задаётся один раз в `build.rs` (`APP_NAME`): заголовок окна, страница
  аккаунта, строка плеера, Discord, MPRIS. Теперь оттуда же читают `make_app.sh`,
  `make_dmg.sh` и `release.sh` — строки больше не дублируются.
- Файл бандла тоже переименован: `Я.Музыка [RS].app`; имя файла dmg осталось латиницей
  (`yamusic-rs-<версия>.dmg`), ярлык `/Applications` внутри тома прежний.
- `CFBundleIdentifier` и метка LaunchAgent остались `ru.spolzermod.yamusic-rs`: смена
  идентификатора сломала бы автозапуск и регистрацию в Launch Services.
- Релиз `1.1.3` обновлён на месте: dmg заменён через `gh release upload --clobber`,
  SHA-256 и заголовок релиза обновлены.

### Подпись бандла ломала запуск скачанного dmg (17.11.2026)

- Gatekeeper писал «is damaged and can't be opened» на скачанном приложении. Причина:
  `make_app.sh` ставил ad-hoc подпись, когда `Contents/Resources/AppIcon.iconset` ещё лежал
  в бандле, а `trap` удалял его уже после `codesign`. Подпись запечатывала эти файлы, они
  исчезали — печать становилась недействительной. Локально ошибка не видна: без карантина
  macOS подпись не проверяет.
- Исправлено: `.iconset` удаляется сразу после `iconutil`, до подписи; `make_dmg.sh`
  проверяет `codesign --verify --strict` и не упаковывает бандл с битой печатью.
- Релиз `1.1.3` перезалит с исправленной подписью (SHA-256 в описании обновлён).

## Слияние с v1.1.3 (20.11.2026)

- База — апстрим `v1.1.3` (`abade9c`), 21 коммит поверх `v1.1.1`. Версия форка — `1.1.4`:
  тег `v1.1.3` уже занят релизом форка, поэтому следующий патч.
- Из upstream интегрировано: волна учится на прослушиваниях (`/plays`, сессии ротора,
  продолжение сессии после перезапуска через `wave.json`), пропуск в волне сразу просит
  новые треки, «Треки 18+» в меню профиля, громкость в мини-плеере, свой логотип,
  перерисовка окна после переноса между мониторами, выключение проверки обновлений,
  удаление трека по ревизии плейлиста.
- `src/stream.rs` оставлен форка: upstream качает трек целиком в память (`Buf`), у нас
  Range-куски с фоновой докачкой и предзагрузкой для перемотки. «Играет, пока качается»
  у нас было и раньше — первый кусок 256 КБ.
- `src/player.rs`: новый слой волны/18+ совмещён с нашим потоковым движком (rodio 0.22,
  coarse seek, микро-фейд). `Audio`/`current_buf` заменены на `HttpStream`/`current_url`,
  поэтому повтор и перемотка не ходят в сеть из аудиопотока.
- Смена выхода Windows (`check_output`) перенесена на наш `HttpStream`: переоткрытие
  потока по сохранённому адресу и предзагрузка целевого куска; код под `#[cfg(windows)]`,
  на macOS не собирается.
- `src/api.rs`: ревизия плейлиста из upstream + наш `take_result` без клонов; `playlist()`
  по-прежнему читает `tracks` срезом, без копии массива.
- `src/net.rs`: `refresh_again` из upstream + наш весовой лимит кэша и `release_prefix`.
- `ui/state.slint`: вместе `repaint` (upstream) и `mac`/`windows`/`gpu`/`chrome-top` (форк).
- `README.md`: раздел про проверку хешей помечен как сборка оригинального проекта; для
  форка добавлена проверка dmg через `shasum -a 256` (SHA-256 есть в описании релиза).
- Проверки на macOS: `cargo clippy --release --all-targets -- -D warnings`,
  `cargo test --release` (29 passed, 5 ignored), `cargo build --release` — зелёные.

### Возврат имени «Яндекс Музыка RS» (20.11.2026)

- Отображаемое имя снова `Яндекс Музыка RS` (было «Я.Музыка [RS]»): `APP_NAME` в `build.rs`,
  описание в `Cargo.toml`, заголовок и инструкции в `README.md`.
- Ярлык и файл бандла снова `Яндекс Музыка RS.app`; dmg остаётся `yamusic-rs-<версия>.dmg`,
  `CFBundleIdentifier` и метка LaunchAgent не меняются.
- На GitHub обновлены описание репозитория и заголовок релиза `v1.1.3`. Slug остаётся
  `yamusic-rs`, чтобы не менять ссылки, `repository` в `Cargo.toml` и обновлялку.
