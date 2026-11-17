#!/bin/bash
# Собирает .dmg из готового .app: том с приложением и симлинком /Applications,
# чтобы установка была перетаскиванием. Сам .app делает tools/make_app.sh.
# Использование: tools/make_dmg.sh [--debug] (по умолчанию release).
set -euo pipefail
cd "$(dirname "$0")/.."

PROFILE=release
if [ "${1:-}" = "--debug" ]; then
  PROFILE=debug
fi

VERSION=$(grep -m1 '^version' Cargo.toml | cut -d'"' -f2)
# Имя берём из того же места, что и make_app.sh, — из build.rs.
NAME=$(sed -n 's/^const APP_NAME: &str = "\(.*\)";/\1/p' build.rs)
APP="target/$PROFILE/$NAME.app"
DMG="target/$PROFILE/yamusic-rs-$VERSION.dmg"
STAGE=$(mktemp -d)
trap 'rm -rf "$STAGE"' EXIT

if [ ! -d "$APP" ]; then
  echo "Нет $APP — сначала соберите его: tools/make_app.sh" >&2
  exit 1
fi

cp -R "$APP" "$STAGE/"
# Симлинк, чтобы приложение перетаскивалось в «Программы» прямо из окна тома.
ln -s /Applications "$STAGE/Applications"
rm -f "$DMG"
hdiutil create -volname "$NAME" -srcfolder "$STAGE" \
  -fs HFS+ -format UDZO -ov -quiet "$DMG"
echo "Готово: $DMG"
