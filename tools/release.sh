#!/bin/bash
# Релиз с локальной сборкой: universal .app → .dmg → релиз на GitHub.
# GitHub-раннеры macOS периодически не выдаются часами, поэтому релиз собирается на маке
# автора, а CI остаётся проверкой (clippy, тесты, сборка).
# Использование: tools/release.sh (версия берётся из Cargo.toml).
set -euo pipefail
cd "$(dirname "$0")/.."

VERSION=$(grep -m1 '^version' Cargo.toml | cut -d'"' -f2)
TAG="v$VERSION"
REMOTE=fork
REPO=$(git remote get-url "$REMOTE" | sed -E 's#.*github\.com[:/](.+)$#\1#' | sed 's#\.git$##')

die() { echo "$*" >&2; exit 1; }

[ -z "$(git status --porcelain)" ] || die "Есть незакоммиченные изменения: релиз собирается только из чистого дерева"
git fetch -q "$REMOTE" main
[ "$(git rev-parse HEAD)" = "$(git rev-parse "$REMOTE/main")" ] || die "HEAD не совпадает с $REMOTE/main — сначала push"
if git rev-parse -q --verify "refs/tags/$TAG" >/dev/null; then
  [ "$(git rev-parse "$TAG^{commit}")" = "$(git rev-parse HEAD)" ] || die "Тег $TAG указывает не на HEAD"
else
  git tag -a "$TAG" -m "Яндекс Музыка RS $VERSION"
fi

MACOS_UNIVERSAL=1 tools/make_app.sh
tools/make_dmg.sh
DMG="target/release/yamusic-rs-$VERSION.dmg"
hdiutil verify -quiet "$DMG"
SHA=$(shasum -a 256 "$DMG" | awk '{print $1}')
PREV=$(git describe --tags --abbrev=0 "$TAG^")

BODY="Сборка universal (Apple Silicon и Intel) из коммита $(git rev-parse --short HEAD).

Изменения с $PREV:
$(git log --format='- %s' "$PREV..HEAD")

SHA-256: $SHA"

git push -q "$REMOTE" "$TAG"
gh release create "$TAG" "$DMG" --repo "$REPO" \
  --title "Яндекс Музыка RS $VERSION" --notes "$BODY"
