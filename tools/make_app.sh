#!/bin/bash
# Собирает .app-бандл для macOS: бинарник, Info.plist и иконка из icons/app.ico.
# Источник иконки — кадр 256×256, крупные размеры растягиваются из него.
# Использование: tools/make_app.sh [--debug] (по умолчанию release).
set -euo pipefail
cd "$(dirname "$0")/.."

PROFILE=release
if [ "${1:-}" = "--debug" ]; then
  PROFILE=debug
fi

# Сборка бинарника. Три режима:
#  - MACOS_UNIVERSAL=1 — обе арки (arm64 + x86_64) и lipo в universal (CI: Intel-раннер);
#  - MACOS_TARGET=<triple> — одна кросс-арка;
#  - иначе — нативно под хост (локальная сборка на Apple Silicon).
VERSION=$(grep -m1 '^version' Cargo.toml | cut -d'"' -f2)
NAME="Яндекс Музыка RS"
APP="target/$PROFILE/yamusic-rs.app"

if [ "${MACOS_UNIVERSAL:-}" = "1" ]; then
  # Кросс-сборка x86_64 нужна только здесь: без std под цель rustc падает на
  # «can't find crate for core», поэтому проверяем заранее и подсказываем лечение.
  LIBDIR=$(rustc --print target-libdir --target x86_64-apple-darwin 2>/dev/null || true)
  if [ -z "$LIBDIR" ] || [ ! -d "$LIBDIR" ]; then
    echo "Нет std под x86_64-apple-darwin: rustup target add x86_64-apple-darwin" >&2
    echo "Если rustc из Homebrew, он цели не видит: поставьте rustup-тулчейн первым в PATH." >&2
    exit 1
  fi
  cargo build "--$PROFILE" --target aarch64-apple-darwin
  cargo build "--$PROFILE" --target x86_64-apple-darwin
  BIN="target/$PROFILE/yamusic-rs-universal"
  lipo -create \
    "target/aarch64-apple-darwin/$PROFILE/yamusic-rs" \
    "target/x86_64-apple-darwin/$PROFILE/yamusic-rs" \
    -output "$BIN"
elif [ -n "${MACOS_TARGET:-}" ]; then
  cargo build "--$PROFILE" --target "$MACOS_TARGET"
  BIN="target/$MACOS_TARGET/$PROFILE/yamusic-rs"
else
  cargo build "--$PROFILE"
  BIN="target/$PROFILE/yamusic-rs"
fi

rm -rf "$APP"
mkdir -p "$APP/Contents/MacOS" "$APP/Contents/Resources"
cp "$BIN" "$APP/Contents/MacOS/yamusic-rs"

# Иконка: app.ico → PNG 256 → все размеры → icns.
ICONSET="$APP/Contents/Resources/AppIcon.iconset"
TMP=$(mktemp -d)
trap 'rm -rf "$TMP" "$ICONSET"' EXIT
mkdir -p "$ICONSET"
sips -s format png icons/app.ico --out "$TMP/base.png" >/dev/null
for n in 16 32 64 128 256 512 1024; do
  sips -z "$n" "$n" "$TMP/base.png" --out "$TMP/$n.png" >/dev/null
done
cp "$TMP/16.png"   "$ICONSET/icon_16x16.png"
cp "$TMP/32.png"   "$ICONSET/icon_16x16@2x.png"
cp "$TMP/32.png"   "$ICONSET/icon_32x32.png"
cp "$TMP/64.png"   "$ICONSET/icon_32x32@2x.png"
cp "$TMP/128.png"  "$ICONSET/icon_128x128.png"
cp "$TMP/256.png"  "$ICONSET/icon_128x128@2x.png"
cp "$TMP/256.png"  "$ICONSET/icon_256x256.png"
cp "$TMP/512.png"  "$ICONSET/icon_256x256@2x.png"
cp "$TMP/512.png"  "$ICONSET/icon_512x512.png"
cp "$TMP/1024.png" "$ICONSET/icon_512x512@2x.png"
iconutil -c icns "$ICONSET" -o "$APP/Contents/Resources/AppIcon.icns"

cat > "$APP/Contents/Info.plist" <<EOF
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
	<key>CFBundleName</key><string>$NAME</string>
	<key>CFBundleDisplayName</key><string>$NAME</string>
	<key>CFBundleExecutable</key><string>yamusic-rs</string>
	<key>CFBundleIdentifier</key><string>ru.spolzermod.yamusic-rs</string>
	<key>CFBundlePackageType</key><string>APPL</string>
	<key>CFBundleShortVersionString</key><string>$VERSION</string>
	<key>CFBundleVersion</key><string>$VERSION</string>
	<key>CFBundleIconFile</key><string>AppIcon</string>
	<key>LSMinimumSystemVersion</key><string>12.0</string>
	<key>NSHighResolutionCapable</key><true/>
	<key>LSApplicationCategoryType</key><string>public.app-category.music</string>
</dict>
</plist>
EOF

# Ad-hoc подпись: без неё передача бандла на другой мак упрётся в Gatekeeper.
codesign --force --sign - "$APP" >/dev/null 2>&1 || echo "codesign не сработал — локально запустится и так"

echo "Готово: $APP"
echo "Запуск: open \"$APP\""
