#!/bin/bash
# Релиз для Apple Silicon с локальной сборкой: .app → .dmg → релиз на GitHub.
# GitHub-раннеры macOS периодически не выдаются часами, поэтому релиз собирается на маке
# автора, а CI остаётся проверкой (clippy, тесты, сборка).
# Использование: tools/release.sh (версия берётся из Cargo.toml).
set -euo pipefail
cd "$(dirname "$0")/.."

VERSION=$(grep -m1 '^version' Cargo.toml | cut -d'"' -f2)
TAG="v$VERSION"
# Имя приложения — из build.rs, как в make_app.sh.
NAME=$(sed -n 's/^const APP_NAME: &str = "\(.*\)";/\1/p' build.rs)
REMOTE=fork
REPO=$(git remote get-url "$REMOTE" | sed -E 's#.*github\.com[:/](.+)$#\1#' | sed 's#\.git$##')

die() { echo "$*" >&2; exit 1; }

[ "$(uname -s)" = Darwin ] && [ "$(uname -m)" = arm64 ] || die "Релиз собирается на Mac с Apple Silicon"
[ -z "$(git status --porcelain)" ] || die "Есть незакоммиченные изменения: релиз собирается только из чистого дерева"
git fetch -q "$REMOTE" main
[ "$(git rev-parse HEAD)" = "$(git rev-parse "$REMOTE/main")" ] || die "HEAD не совпадает с $REMOTE/main — сначала push"
if git rev-parse -q --verify "refs/tags/$TAG" >/dev/null; then
  [ "$(git rev-parse "$TAG^{commit}")" = "$(git rev-parse HEAD)" ] || die "Тег $TAG указывает не на HEAD"
else
  git tag -a "$TAG" -m "$NAME $VERSION"
fi

MACOS_UNIVERSAL=0 MACOS_TARGET= tools/make_app.sh
tools/make_dmg.sh
DMG="target/release/yamusic-rs-$VERSION-macos-arm64.dmg"
hdiutil verify -quiet "$DMG"
SHA=$(shasum -a 256 "$DMG" | awk '{print $1}')
# Только теги релизов: служебные (вроде pre-merge-…) не должны становиться
# базой списка изменений.
PREV=$(git describe --tags --abbrev=0 --match 'v*' "$TAG^" 2>/dev/null || true)
if [ -n "$PREV" ]; then
  CHANGES="Изменения с $PREV:
$(git log --format='- %s' "$PREV..HEAD")"
else
  CHANGES="$(git log -1 --format=%s HEAD)"
fi

BODY="Сборка для Apple Silicon (arm64) из коммита $(git rev-parse --short HEAD).

$CHANGES

SHA-256: $SHA"

git push -q "$REMOTE" "$TAG"
gh release create "$TAG" "$DMG" --repo "$REPO" \
  --title "$NAME $VERSION для Apple Silicon" --notes "$BODY"
