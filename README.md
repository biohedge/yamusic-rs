<div align="center">

<img src="docs/logo.png" width="96" height="96" alt="">

# Яндекс Музыка RS

**Интерфейс официального приложения Яндекс Музыки на Rust: один файл и в 13 раз меньше памяти.**

[![Релиз](https://img.shields.io/github/v/release/biohedge/yamusic-rs?label=%D1%80%D0%B5%D0%BB%D0%B8%D0%B7&color=ffd60a&labelColor=1c1c1f)](https://github.com/biohedge/yamusic-rs/releases/latest)
![Windows 10 | 11](https://img.shields.io/badge/Windows-10%20%7C%2011-2b2b30?labelColor=1c1c1f)
![Linux бета](https://img.shields.io/badge/Linux-%D0%B1%D0%B5%D1%82%D0%B0-2b2b30?labelColor=1c1c1f)
![Android бета](https://img.shields.io/badge/Android-%D0%B1%D0%B5%D1%82%D0%B0-2b2b30?labelColor=1c1c1f)
[![Поддержать](https://img.shields.io/badge/%D0%BF%D0%BE%D0%B4%D0%B4%D0%B5%D1%80%D0%B6%D0%B0%D1%82%D1%8C-CloudTips-ffd60a?labelColor=1c1c1f)](https://pay.cloudtips.ru/p/befb620d)
[![GPL-3.0](https://img.shields.io/badge/%D0%BB%D0%B8%D1%86%D0%B5%D0%BD%D0%B7%D0%B8%D1%8F-GPL--3.0-2b2b30?labelColor=1c1c1f)](LICENSE)

[Скачать](https://github.com/biohedge/yamusic-rs/releases/latest) ·
[Возможности](#возможности) ·
[Замеры](#замеры) ·
[Какие системы](#какие-системы-поддерживаются) ·
[Вопросы](#частые-вопросы) ·
[Как помочь](#как-помочь) ·
[Для разработчиков](#для-разработчиков) ·
[English](#english)

<br>

<img src="docs/compare.gif" width="100%" alt="Шторка: слева официальное приложение, справа Яндекс Музыка RS. Страницы альбома и исполнителя в светлой и тёмной теме">

<sub>Слева официальное приложение, справа RS. Аккаунт, страница и момент одни и те же.</sub>

</div>

<br>

Официальное приложение Яндекс Музыки для Windows сделано на Electron: внутри браузер Chromium,
семь процессов и около 850 МБ памяти. **Яндекс Музыка RS** повторяет его интерфейс (те же
шрифты, цвета, отступы и страницы), но написана на Rust и рисует окно сама, на процессоре.
В итоге один файл на 26 МБ, один процесс и около 65 МБ памяти. Работает на Windows, а в бете
на Linux и Android. Этот форк собирает и публикует сборку для macOS.

<div align="center">
<img src="docs/stats.gif" width="100%" alt="Оперативная память: 852 МБ против 64 МБ. Процессы: 7 против 1. Размер на диске: 356 МБ против 26 МБ">
</div>

<sub>Память — в конце одинакового сценария: играет трек, открыт текст песни. Как мерили, показано
[ниже](#замеры).</sub>

> [!IMPORTANT]
> Это неофициальный проект. Он не связан с Яндексом, и Яндекс его не поддерживает. Названия
> и товарные знаки принадлежат их владельцам. Нужен ваш аккаунт Яндекса, а для полных треков —
> подписка Плюс, как и в официальном приложении.

> [!NOTE]
> Здесь публикуется `.dmg` для Mac с Apple Silicon. Порт macOS основан на
> [rnikitin/yamusic-rs](https://github.com/rnikitin/yamusic-rs). Сборки для Windows, Linux и Android
> выпускает [оригинальный проект](https://github.com/SpolzerMod/yamusic-rs).

## Возможности

<img src="docs/lyrics.gif" width="100%" alt="Полноэкранный плеер RS: обложка слева, справа текст песни прокручивается вместе с треком">

- «Моя волна» с колесом вайбов и настройками «по характеру», «под настроение» и «по языку»,
  «Для вас и Тренды», «Концерты», «Книги и подкасты», чарт, поиск, коллекция и история прослушиваний.
- Страницы исполнителей, альбомов и плейлистов. Свои плейлисты можно переименовывать, менять им
  обложку и описание, удалять из них треки. Галочка в меню трека показывает, что он уже в плейлисте.
- Полноэкранный плеер с текстом песни, который идёт вместе с треком.
- Плеер показывает формат трека. В настройках можно включить плавный переход между треками
  и движение пятна «Моей волны».
- Плеер запоминает трек и позицию через Ynison, чтобы продолжить прослушивание здесь
  или в официальном приложении.
- Скачивание треков, альбомов и плейлистов с обложкой и тегами. В настройках можно выбрать папку
  и включить скачивание FLAC при наличии lossless.
- Эквалайзер на 10 полос, логарифмическая громкость, светлая и тёмная тема, русский, английский,
  узбекский и казахский языки.
- Автозапуск вместе с системой. На Windows есть трей: если закрыть окно во время игры, музыка
  не остановится.
- Мини-плеер поверх всех окон: обложка, название и три кнопки. Включается кнопкой рядом
  с «Свернуть».
- Медиаклавиши клавиатуры работают, даже когда окно свёрнуто.
- Статус «Слушает …» в Discord и переключатель «Меньше ИИ» для рекомендаций.
- Обновление из приложения: когда выходит новая версия, сверху появляется кнопка «Обновить».
- Режим «Лёгкий» без анимаций для слабых компьютеров.

## Замеры

<img src="docs/live.gif" width="100%" alt="Графики памяти и процессора за 50 секунд. Официальное приложение держится около 800–850 МБ, RS — около 60 МБ. Процессор в среднем 2,9% против 0,4%">

Оба приложения по очереди проходят один и тот же сценарий, клики идут по таймеру:

1. запуск;
2. на 10-й секунде — «Для вас и Тренды» и прокрутка вниз;
3. на 24-й — включается трек;
4. на 32-й — полноэкранный плеер с текстом песни.

Каждые полсекунды снимаются Private Bytes и процессорное время всех процессов приложения. Окна видны
на экране, звук выключен.

<sub>Windows 11, Intel Core i3-13100, монитор 2560×1440 с масштабом 125%, окно 1280×800. Процессор
считается в процентах от всего процессора, как в диспетчере задач. Размер на диске — папка установленного
официального приложения против одного `yamusic-rs.exe`. На другом компьютере и с другими страницами
цифры будут другими, но разница того же порядка.</sub>

## Установка

### Windows (оригинальный проект)

Этот форк exe не публикует: сборки для Windows лежат в
[релизах оригинального проекта](https://github.com/SpolzerMod/yamusic-rs/releases/latest).

1. Скачайте `yamusic-rs.exe` со страницы [последнего релиза](https://github.com/SpolzerMod/yamusic-rs/releases/latest).
2. Положите файл в любую папку, например в `Документы` или `C:\Programs`, и запустите.
   Установщика нет, приложение целиком помещается в один exe.
3. Приложение покажет код. Откройте [ya.ru/device](https://ya.ru/device) на компьютере или телефоне
   и введите его. Пароль в приложение вводить не нужно.

> [!NOTE]
> У exe нет цифровой подписи, поэтому при первом запуске Windows SmartScreen может показать «Windows
> защитила ваш компьютер». Нажмите «Подробнее», затем «Выполнить в любом случае». Контрольная сумма
> SHA-256 файла есть на странице релиза. Exe собирается на GitHub Actions прямо из исходников,
> лог сборки открыт. Можно собрать и самим ([как это сделать](#сборка)).

**Обновление.** Приложение раз в полчаса проверяет релизы на GitHub. Кнопка «Обновить» скачивает
новый `yamusic-rs.exe`, сверяет его SHA-256 с релизом, заменяет старый файл и перезапускается.
Если папка закрыта для записи (например, `Program Files`), кнопка откроет страницу релиза,
и файл можно заменить вручную.

**Удаление.**
1. Выключите в настройках «Запускать вместе с Windows», если включали.
2. Удалите `yamusic-rs.exe`.
3. Удалите папку `%APPDATA%\yamusic-rs` с настройками, входом и кэшем.

Скачанные треки лежат в `Музыка\Яндекс Музыка`, их удаляйте по желанию.

### macOS (Apple Silicon)

1. Скачайте `yamusic-rs-<версия>-macos-arm64.dmg` со страницы
   [последнего релиза](https://github.com/biohedge/yamusic-rs/releases/latest).
2. Откройте dmg и перетащите `Яндекс Музыка RS.app` в «Программы»: ярлык «Программы» лежит рядом
   в окне тома.
3. Приложение покажет код. Откройте [ya.ru/device](https://ya.ru/device) на компьютере или телефоне
   и введите его.

Приложение без подписи Apple, поэтому при первом запуске macOS может его заблокировать: нажмите
на `.app` правой кнопкой → «Открыть» → «Открыть». Из Терминала то же самое делается командой
`xattr -d com.apple.quarantine "/Applications/Яндекс Музыка RS.app"`. Сборка работает на
Apple Silicon (M1 и новее). Автозапуск при входе включается в настройках
(LaunchAgent, обычное окно).

**Обновление.** Приложение сообщает о новой версии, кнопка «Обновить» открывает страницу релиза:
новый dmg ставится поверх старого перетаскиванием.

**Удаление.** Удалите `Яндекс Музыка RS.app` из «Программ» и папку
`~/Library/Application Support/yamusic-rs`.

### Linux, бета (оригинальный проект)

Этот форк пакеты не публикует: `.deb`, `.rpm` и AppImage лежат в
[релизах оригинального проекта](https://github.com/SpolzerMod/yamusic-rs/releases/latest).

В релизе три файла, внутри одна и та же программа. Выберите тот, что подходит вашей системе:

| Система | Файл | Установка |
| --- | --- | --- |
| Debian, Ubuntu, Mint и производные | `yamusic-rs_<версия>-1_amd64.deb` | `sudo apt install ./yamusic-rs_*.deb` |
| Fedora, openSUSE | `yamusic-rs-<версия>-1.x86_64.rpm` | `sudo dnf install ./yamusic-rs-*.rpm` или `sudo zypper install ./yamusic-rs-*.rpm` |
| Любая другая, в том числе Arch | `yamusic-rs-x86_64.AppImage` | `chmod +x yamusic-rs-x86_64.AppImage` и запустить |

Для ARM (Raspberry Pi 4/5, Asahi Linux на Mac с M1/M2 и т. п.) те же файлы с `arm64`/`aarch64` в имени.

AppImage монтирует себя через FUSE. Если пишет `Cannot mount AppImage`, поставьте пакет `fuse3`
или запускайте с распаковкой: `APPIMAGE_EXTRACT_AND_RUN=1 ./yamusic-rs-x86_64.AppImage`.

Для Arch есть и [`PKGBUILD`](packaging/aur/PKGBUILD): он собирает пакет из `.deb` релиза (`makepkg -si`).

Приложение появится в меню как «Яндекс Музыка RS». Вход такой же, по коду на ya.ru/device.

**Обновление.** Приложение сообщает о новой версии, кнопка «Обновить» открывает страницу релиза.
Пакет ставится поверх старого той же командой, AppImage заменяется новым файлом.

**Удаление.** `sudo apt remove yamusic-rs`, `sudo dnf remove yamusic-rs` или удалить AppImage. Настройки
и вход лежат в `~/.config/yamusic-rs`, кэш в `~/.cache/yamusic-rs`, токен в связке ключей под
именем «Яндекс Музыка RS» (его можно удалить в «Паролях и ключах» или в KWallet).

### Android (бета)

1. Скачайте `yamusic-rs-arm64-v8a.apk` со страницы [последнего релиза](https://github.com/SpolzerMod/yamusic-rs/releases/latest)
   прямо на телефон.
2. Откройте файл. Android попросит разрешить установку из этого источника (браузера или
   файлового менеджера): разрешите и установите.
3. Вход такой же, по коду: откройте [ya.ru/device](https://ya.ru/device) на любом устройстве и введите
   код с экрана.

Нужен Android 8 или новее на 64-битном процессоре, то есть почти любой телефон последних лет.
APK весит около 11 МБ. Играет в фоне, управляется из шторки и с экрана блокировки, скачанные
треки кладёт в `Музыка/Яндекс Музыка`.

**Обновление.** В профиле появится «Доступна версия …». Кнопка скачает новый APK, его нужно
открыть и установить поверх, вход и настройки сохранятся.

## Какие системы поддерживаются

| Система | Состояние |
| --- | --- |
| **macOS** 12+ (Apple Silicon) | `.dmg` в [релизах](https://github.com/biohedge/yamusic-rs/releases). Нативный «светофор» и скруглённые углы, медиа-клавиши и «Сейчас играет» в Пункте управления. Автозапуск при входе — в настройках, через LaunchAgent. |
| **Windows** 10 и 11 x64 | Основная платформа [оригинального проекта](https://github.com/SpolzerMod/yamusic-rs), проверяется каждая версия. В этом форке не собирается. |
| **Linux** x86_64 и ARM64 | Бета в [оригинальном проекте](https://github.com/SpolzerMod/yamusic-rs). |
| **Android** 8+ ARM64 | Бета в [оригинальном проекте](https://github.com/SpolzerMod/yamusic-rs). |

**Нужно ещё:**
- аккаунт Яндекса. Без Плюса сервер отдаёт только 30-секундные отрывки;
- интернет. Офлайн-режима, как в мобильном приложении, нет;
- шрифты YS Text и иконки. Если официальное приложение установлено, RS берёт их из него.
  Если нет, один раз скачивает с music.yandex.ru в `%APPDATA%\yamusic-rs\assets`
  (на macOS в `~/Library/Application Support/yamusic-rs/assets`, на Linux в `~/.cache/yamusic-rs/assets`).
  Официального приложения под Linux нет, поэтому
  там интерфейс пока только на русском: словари других языков есть только в нём.

## Частые вопросы

<details>
<summary><b>Это безопасно? Куда уходит мой токен?</b></summary>

<br>

Вход идёт по коду устройства через ya.ru/device, пароль приложение не видит. Полученный токен
шифруется средствами Windows (DPAPI) и хранится в `%APPDATA%\yamusic-rs\config.json`. Расшифровать
его может только ваша учётная запись Windows на этом компьютере.

На Linux токен лежит в связке ключей (GNOME Keyring, KWallet или другая служба Secret Service),
а в `~/.config/yamusic-rs/config.json` остаётся только имя записи. Если связки ключей нет, например
в голом оконном менеджере, токен хранится в этом файле, который может читать только ваш пользователь.

На macOS токен лежит открытым текстом в `~/Library/Application Support/yamusic-rs/config.json`.
Файл может читать только ваш пользователь.

Токен отправляется только по https и только на домены `yandex.ru` и `yandex.net`. Проверка
встроена в код (`src/client.rs`). Кроме Яндекса приложение ходит только к `api.github.com`,
чтобы проверить обновления, и к Discord на этом же компьютере (локальный канал), если включён
статус. Аналитики и телеметрии нет. Проверку обновлений можно выключить в настройках.

Если вы больше не доверяете приложению или потеряли компьютер, токен можно отозвать: на
[id.yandex.ru](https://id.yandex.ru/security) в разделе «Безопасность» откройте «Входы и устройства»
и выйдите на нужном устройстве или сразу на всех. После этого токен перестанет работать, а RS
попросит войти заново. Выход из аккаунта в самом RS только стирает токен с компьютера.
</details>

<details>
<summary><b>Как проверить, что файл собран из этих исходников?</b></summary>

<br>

В оригинальном проекте релизы для Windows и Linux собирает GitHub Actions из кода тега,
руками файлы не загружаются. В логе сборки
(вкладка Actions, шаг «SHA-256») напечатан хеш получившегося `yamusic-rs.exe`, `.deb` и
бинарника для Linux. Он совпадает с хешем файла в релизе, а GitHub показывает его рядом с каждым
файлом.

Сборка воспроизводимая: два прогона одного и того же кода дают побайтно одинаковые exe, бинарник
и `.deb` (`.rpm` и AppImage — нет, в них записано время сборки). Самый честный способ проверить:
сделать форк, запустить в нём сборку на том же теге и сравнить хеши. Учтите, что в exe записан
путь к файлу отладки с именем репозитория, поэтому для exe сравнение сработает, только если форк
называется так же — `yamusic-rs`.

Собрать то же самое у себя на компьютере не выйдет: в CI другая версия линкера MSVC, и в exe
попадают пути к папке `.cargo`. Хеш совпадёт только у сборки в GitHub Actions.

Хеш скачанного файла на Windows:

```powershell
Get-FileHash .\yamusic-rs.exe -Algorithm SHA256
```

В форке macOS-сборка делается локально, а SHA-256 файла dmg напечатан в описании релиза.
Проверить скачанный образ:

```bash
shasum -a 256 ~/Downloads/yamusic-rs-<версия>-macos-arm64.dmg
```
</details>

<details>
<summary><b>Почему так мало памяти?</b></summary>

<br>

Официальное приложение — это сайт внутри Chromium: отдельные процессы для окна, страницы, GPU,
звука и расширений плюс движок JavaScript. RS — обычная программа на Rust. Интерфейс описан
на [Slint](https://slint.dev) и рисуется библиотекой [Skia](https://skia.org) на процессоре,
без видеокарты. Видеокарта сделала бы прокрутку чуть дешевле, но драйвер сразу забирает
100–150 МБ, поэтому от неё отказались. Обложки декодируются ровно под размер на экране и живут
в кэше с ограничением по объёму.
</details>

<details>
<summary><b>Чем RS отличается от официального приложения?</b></summary>

<br>

Внешне почти ничем, это видно на сравнении выше. Чего пока нет или что работает иначе:

- на «Для вас и Тренды» показываются не все блоки: например, «Подборки музыки» и «Артист рекомендует»
  пока пропускаются;
- нет выбора города для концертов и загрузки своих треков;
- на Windows нет панели управления в системном меню громкости (на Linux она есть через MPRIS);
- собственные настройки RS («Режим работы», «Статус в Discord», автозапуск) переведены только на русский.

Есть и то, чего нет в оригинале: режим «Лёгкий», статус в Discord, автозапуск в трее.
</details>

<details>
<summary><b>Где посмотреть, что пошло не так?</b></summary>

<br>

Ошибки пишутся в `%APPDATA%\yamusic-rs\log.txt`, на Linux в `~/.config/yamusic-rs/log.txt`.
API Яндекса неофициальный и может меняться без предупреждения.
Как сообщить о проблеме, написано [ниже](#как-помочь).
</details>

<details>
<summary><b>Можно запускать сразу свёрнутым? А если запустить второй раз?</b></summary>

<br>

`yamusic-rs.exe --tray` запускает приложение сразу в трее, так же работает автозапуск. Второй
запуск не открывает вторую копию, а показывает уже открытое окно. На Linux трея пока нет, поэтому
автозапуск открывает окно, а `--tray` ничего не делает. Второй запуск так же показывает открытое окно.
</details>

## Как помочь

### Поддержать рублём

RS бесплатный и без рекламы. Если он вам пригодился, можно закинуть на развитие через
[CloudTips](https://pay.cloudtips.ru/p/befb620d), от 49 ₽. Деньги пойдут на RS и следующие открытые проекты.

### Сообщить об ошибке

Откройте [issue](https://github.com/biohedge/yamusic-rs/issues/new/choose) и напишите:

- версию RS из «Настройки → О приложении» и версию системы (на Linux ещё дистрибутив и окружение: GNOME, KDE…);
- что вы делали, что ожидали увидеть и что получилось;
- последние строки `%APPDATA%\yamusic-rs\log.txt` (на Linux `~/.config/yamusic-rs/log.txt`),
  если там есть что-то про эту ошибку.

Токена в логе нет, но в адресах запросов может встретиться числовой id вашего аккаунта. Его можно
заменить на `***`. Со скриншотами то же самое: закрепы и рекомендации показывают ваши вкусы,
их можно закрасить.

Если вы нашли расхождение с официальным приложением, приложите два скриншота при одинаковом
размере окна: оригинал и RS.

### Критические ошибки

Если ошибка касается безопасности, **не открывайте публичный issue**. Это, например:

- токен уходит куда-то кроме `yandex.ru` и `yandex.net` или попадает в лог;
- обновлялка ставит файл, который не совпадает с релизом;
- другая программа или другой пользователь компьютера может прочитать токен.

Напишите через [Security → Report a vulnerability](https://github.com/biohedge/yamusic-rs/security/advisories/new).
Сообщение увидят только владельцы репозитория. Подробнее в [SECURITY.md](SECURITY.md).

Если после обновления приложение перестало запускаться, это тоже срочно, но писать можно
в обычный issue, в заголовке укажите версию. Пока ждёте исправления, предыдущую версию можно
скачать со страницы [релизов](https://github.com/biohedge/yamusic-rs/releases).

### Прислать код

- Одна правка — один pull request. Крупное изменение лучше сначала обсудить в issue.
- Перед отправкой должны проходить `cargo clippy --release --all-targets -- -D warnings`
  и `cargo test --release`. То же самое запускает CI.
- Код и комментарии пишутся на русском, интерфейс обращается к пользователю на «вы».
- `ui/theme.slint` и `src/palette.rs` руками не правятся, они генерируются скриптами из `tools/`.
- Если сломался запрос после обновления Яндекса, обычно достаточно поправить `src/client.rs`
  или разбор ответа в `src/api.rs`.

## Для разработчиков

### Сборка

Нужен Rust 1.92 или новее (`rustup update stable`). Дальше на любой системе одинаково:

```bash
git clone https://github.com/biohedge/yamusic-rs
cd yamusic-rs
cargo build --release
```

Первая сборка занимает 5–10 минут, в основном из-за Skia и LTO. Для отладки подойдёт
`cargo run`: в debug-сборке лог дублируется в консоль.

#### Windows

Нужны Visual Studio Build Tools с компонентом «Разработка классических приложений на C++»
(MSVC и Windows SDK: из SDK берётся `rc.exe` для значка и описания exe).
Готовый файл: `target\release\yamusic-rs.exe`.

#### Linux

Нужны компилятор C и заголовки библиотек.

Debian, Ubuntu, Mint:

```bash
sudo apt install build-essential pkg-config libasound2-dev libfontconfig1-dev libfreetype-dev libwayland-dev libxkbcommon-dev libxkbcommon-x11-dev libx11-dev libx11-xcb-dev libxcursor-dev libxi-dev libxrandr-dev
```

Fedora:

```bash
sudo dnf install gcc gcc-c++ make pkgconf-pkg-config alsa-lib-devel fontconfig-devel freetype-devel wayland-devel libxkbcommon-devel libxkbcommon-x11-devel libX11-devel libXcursor-devel libXi-devel libXrandr-devel
```

Arch:

```bash
sudo pacman -S --needed base-devel alsa-lib fontconfig freetype2 wayland libxkbcommon libxkbcommon-x11 libx11 libxcursor libxi libxrandr
```

Готовый файл: `target/release/yamusic-rs`. Пакеты собираются из него: `cargo deb --no-build`
([cargo-deb](https://github.com/kornelski/cargo-deb)), `cargo generate-rpm`
([cargo-generate-rpm](https://github.com/cat-in-136/cargo-generate-rpm)) и `tools/appimage.sh`.
Релизы собираются на Ubuntu 22.04: собранное на более новой системе потребует её версию glibc.

#### macOS

Нужны Xcode Command Line Tools. Собрать `.app`, запустить и собрать `.dmg`:

```bash
tools/make_app.sh     # соберёт target/release/Яндекс Музыка RS.app (--debug — отладочный)
open "target/release/Яндекс Музыка RS.app"
tools/make_dmg.sh     # target/release/yamusic-rs-<версия>-macos-arm64.dmg на Apple Silicon
```

Для быстрой проверки без бандла подойдёт `cargo run`. Для локальной universal-сборки
можно запустить `MACOS_UNIVERSAL=1 tools/make_app.sh`. Для кросс-сборки нужен std
под x86_64 (`rustup target add x86_64-apple-darwin`); `rustc` из Homebrew целей rustup
не видит, поэтому берите тулчейн rustup.

#### Если Skia не скачивается

`skia-bindings` при сборке скачивает готовые бинарники Skia. Если не получается (корпоративный
прокси, ошибка curl), скачайте архив для своей системы с
[rust-skia/skia-binaries](https://github.com/rust-skia/skia-binaries/releases) и укажите путь
в переменной окружения:

```bash
SKIA_BINARIES_URL=file:///путь/к/skia.tar.gz cargo build --release
```

На Windows путь пишется как `file://C:/путь/к/skia.tar.gz`.

### Как устроено

| Файл | Что внутри |
| --- | --- |
| `src/main.rs` | Точка входа: один экземпляр на пользователя, рендерер Skia (по умолчанию без GPU). |
| `src/gui.rs`, `src/gui/landing_ui.rs` | Связь интерфейса с данными: страницы, меню, настройки, обновления. |
| `src/api.rs` | Запросы к API: каталог, лайки, плейлисты, волна, стримы. |
| `src/client.rs` | Всё, что Яндекс может поменять: OAuth-клиент, ключи подписи, заголовки. Ломается — правим здесь. |
| `src/player.rs` | Аудиодвижок в своём потоке: очередь, предзагрузка, сессии волны, фидбек. |
| `src/landing.rs` | Страницы из серверных блоков («Для вас», «Концерты», «Книги и подкасты»). |
| `src/assets.rs`, `src/i18n.rs` | Шрифты, спрайт иконок и словари переводов из `app.asar` оригинала или с music.yandex.ru. |
| `src/images.rs`, `src/net.rs` | Фоновая загрузка обложек и данных страниц, кэши с бюджетом памяти. |
| `src/sys/windows.rs`, `src/tray.rs` | Окна, трей, автозапуск и прочий Win32. |
| `src/sys/linux.rs` | То же для Linux: пути XDG, один экземпляр через сокет, диалоги, автозапуск. |
| `src/sys/macos.rs` | То же для macOS: пути в ~/Library, диалоги osascript, автозапуск LaunchAgent, светофор. |
| `src/mac.rs` | Медиаклавиши и «Сейчас играет» на macOS. |
| `src/mpris.rs` | Медиаклавиши и плеер в панели Linux (MPRIS по D-Bus). |
| `src/secret.rs` | Где хранится токен: DPAPI на Windows, связка ключей на Linux, открытый текст на macOS. |
| `src/discord.rs` | Rich Presence через локальный канал Discord, без сторонних библиотек. |
| `src/update.rs` | Проверка релизов на GitHub и замена exe на лету (на Linux и macOS — ссылка на релиз). |
| `ui/theme.slint`, `src/palette.rs` | Сгенерированы `tools/gen_theme.py` и `tools/gen_palette.py` из CSS и JS оригинала. Руками не править. |

Приложение не перерисовывали на глаз. Цвета, размеры и отступы
сняты из CSS официального приложения скриптами в `tools/` (их запускают на распакованном
`app.asar`: `npx @electron/asar extract app.asar app`). Новые экраны сверяются с оригиналом:
его запускают с `--remote-debugging-port`, через DevTools Protocol снимают размеры DOM и скриншот
1280×800, а затем сравнивают с окном RS попиксельно.

Страницы собирает сервер, как у оригинала: `GET /landing/skeleton/<id>` возвращает список блоков
с типами и адресами данных, и каждый тип блока — свой компонент в `ui/landing.slint`. Неизвестные
типы пропускаются, поэтому новые блоки Яндекса не ломают страницу.

«Моя волна» работает через сессии ротора (`/rotor/session/new`, `/tracks`, `/feedback`), с настройками
из `/rotor/wave/last`. Пропуски и дослушанные треки уходят вместе со следующим запросом треков,
чтобы сервер подбирал продолжение уже с их учётом.

Шрифты, иконки и переводы не лежат в репозитории. `assets.rs` читает их из `app.asar`
установленного оригинала (WOFF переводится в sfnt на лету), а если оригинала нет, скачивает
с веб-версии. Словари ru/en/uz/kk — это чанки webpack оригинала, ключом служит русская строка.

### Обновление из оригинального проекта

Один раз добавьте оригинальный проект как отдельный remote:

```bash
git remote add --no-tags upstream https://github.com/SpolzerMod/yamusic-rs.git
```

В чистой ветке `main` загрузите конкретный релиз и слейте его с текущим кодом. Замените
`v1.1.6` на нужный тег. Так в истории сохраняется связь с апстримом, а локальные теги
остаются тегами релизов форка.

```bash
git fetch --no-tags upstream refs/tags/v1.1.6
git merge --no-ff FETCH_HEAD
```

### Релиз

Для публикации настройте remote `fork` на свой репозиторий GitHub. Релиз собирается
локально и загружается одной командой:

```bash
tools/release.sh
```

Скрипт проверяет, что дерево чистое и `HEAD` уже в `fork/main`, собирает `.app` на Mac с Apple Silicon,
упаковывает его в `yamusic-rs-<версия>-macos-arm64.dmg`, сверяет образ
(`hdiutil verify`), создаёт тег `v<версия>` и публикует релиз с dmg и списком изменений.
Версия берётся из `Cargo.toml`.

GitHub Actions ([`.github/workflows/build.yml`](.github/workflows/build.yml)) на каждый push,
pull request и тег прогоняет clippy, тесты и сборку `.app` + `.dmg`, но релизы не публикует:
macOS-раннеры у GitHub периодически недоступны часами. Сборки для Windows, Linux и Android
публикует оригинальный проект. Встроенная обновлялка ищет последний
релиз на GitHub, а на macOS кнопка «Обновить» открывает страницу релиза.

## Лицензия

Copyright © 2026 Spolzer. Код распространяется по лицензии [GNU GPL 3.0](LICENSE): его можно
использовать, менять и распространять, но изменённая версия, которую вы раздаёте, должна выходить
под той же лицензией и с исходным кодом.

Шестерёнка в логотипе (`icons/app.svg` и собранные из него значки) — логотип Rust от Rust Foundation,
он распространяется по лицензии [CC-BY 4.0](https://creativecommons.org/licenses/by/4.0/).
Шрифты, иконки интерфейса и словари официального приложения в репозиторий не входят: RS читает их
с вашего компьютера или с music.yandex.ru.

## English

**Yandex Music RS** is an unofficial Yandex Music client for macOS written in Rust; the original
project also builds for Windows, Linux and Android. It mirrors the official desktop app closely (same fonts,
colours, layout and pages), but instead of Electron it is a single 26 MB executable that renders
its window on the CPU with Slint and Skia. Running the same scenario, the official app uses about
850 MB of RAM across 7 processes, and RS uses about 65 MB in one process.

This fork ships macOS builds only.

- **macOS:** download `yamusic-rs-<version>-macos-arm64.dmg` from the
  [latest release](https://github.com/biohedge/yamusic-rs/releases/latest), open it and drag
  `Яндекс Музыка RS.app` into Applications. The bundle runs on Apple Silicon. It is
  unsigned, so the first launch needs right click → Open. There is no tray icon; launch at login
  can be turned on in settings (LaunchAgent). Sign in with the code shown at [ya.ru/device](https://ya.ru/device).
- **Windows and Linux:** published by the [original project](https://github.com/SpolzerMod/yamusic-rs):
  `yamusic-rs.exe` for Windows, `.deb` / `.rpm` / AppImage for Linux (beta).
- **Android (beta):** `yamusic-rs-arm64-v8a.apk` for Android 8+ from the
  [original project's release](https://github.com/SpolzerMod/yamusic-rs/releases/latest).
- **Requirements:** macOS 12+ on Apple Silicon, a Yandex account, and Yandex Plus for
  full-length tracks.
- **Privacy:** the token is encrypted with Windows DPAPI, kept in the Linux keyring (Secret Service),
  or stored in the config file on macOS (readable only by your user). It is only sent over HTTPS to
  `yandex.ru` / `yandex.net`. Besides Yandex, the app only talks to `api.github.com` (update checks)
  and the local Discord client (optional status). There is no telemetry.
- **Build:** Rust 1.92+ and Xcode Command Line Tools, then `tools/make_app.sh` and `tools/make_dmg.sh`;
  `cargo build --release` builds the bare binary.
- **Bugs:** open an [issue](https://github.com/biohedge/yamusic-rs/issues/new/choose) with the app
  version, your macOS version and the last lines of
  `~/Library/Application Support/yamusic-rs/log.txt`. Security
  problems go to [Report a vulnerability](https://github.com/biohedge/yamusic-rs/security/advisories/new),
  not to public issues.
- **Support:** if the app is useful to you, you can [send a tip via CloudTips](https://pay.cloudtips.ru/p/befb620d).

The code is released under the [GNU GPL v3](LICENSE): forks you distribute must stay open source
under the same license. This project is not affiliated with or endorsed by Yandex. All trademarks
belong to their owners.
