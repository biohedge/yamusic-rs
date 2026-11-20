#!/bin/bash
# Релиз с локальной сборкой: universal .app → .dmg → релиз на GitHub.
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

# Universal-сборке нужны std под x86_64 и версия не ниже MSRV. Homebrew-сборка Rust
# игнорирует цели rustup, поэтому при необходимости берём подходящий rustup-тулчейн.
REQ=$(grep -m1 '^rust-version' Cargo.toml | cut -d'"' -f2)
version_ok() {
  local v=$1 vmin=${1#*.} rmin=${REQ#*.}
  vmin=${vmin%%.*}
  rmin=${rmin%%.*}
  [ "${v%%.*}" -gt "${REQ%%.*}" ] || { [ "${v%%.*}" -eq "${REQ%%.*}" ] && [ "$vmin" -ge "$rmin" ]; }
}
rustc_ok() {
  local v
  v=$("$1" --version 2>/dev/null | awk '{print $2}')
  [ -n "$v" ] && version_ok "$v" && [ -d "$("$1" --print target-libdir --target x86_64-apple-darwin 2>/dev/null || true)" ]
}
if ! rustc_ok "$(command -v rustc)"; then
  for tc in $(rustup toolchain list 2>/dev/null | awk '{print $1}'); do
    bin="$HOME/.rustup/toolchains/$tc/bin"
    if [ -x "$bin/rustc" ] && rustc_ok "$bin/rustc"; then
      export PATH="$bin:$PATH"
      break
    fi
  done
fi
rustc_ok "$(command -v rustc)" || die "Нужен rustc $REQ+ со std под x86_64: rustup target add x86_64-apple-darwin"

[ -z "$(git status --porcelain)" ] || die "Есть незакоммиченные изменения: релиз собирается только из чистого дерева"
git fetch -q "$REMOTE" main
[ "$(git rev-parse HEAD)" = "$(git rev-parse "$REMOTE/main")" ] || die "HEAD не совпадает с $REMOTE/main — сначала push"
if git rev-parse -q --verify "refs/tags/$TAG" >/dev/null; then
  [ "$(git rev-parse "$TAG^{commit}")" = "$(git rev-parse HEAD)" ] || die "Тег $TAG указывает не на HEAD"
else
  git tag -a "$TAG" -m "$NAME $VERSION"
fi

MACOS_UNIVERSAL=1 tools/make_app.sh
tools/make_dmg.sh
DMG="target/release/yamusic-rs-$VERSION.dmg"
hdiutil verify -quiet "$DMG"
SHA=$(shasum -a 256 "$DMG" | awk '{print $1}')
# Только теги релизов: служебные (вроде pre-merge-…) не должны становиться
# базой списка изменений.
PREV=$(git describe --tags --abbrev=0 --match 'v*' "$TAG^")

BODY="Сборка universal (Apple Silicon и Intel) из коммита $(git rev-parse --short HEAD).

Изменения с $PREV:
$(git log --format='- %s' "$PREV..HEAD")

SHA-256: $SHA"

git push -q "$REMOTE" "$TAG"
gh release create "$TAG" "$DMG" --repo "$REPO" \
  --title "$NAME $VERSION" --notes "$BODY"
