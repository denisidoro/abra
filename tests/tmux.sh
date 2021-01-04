#!/usr/bin/env bash
set -euo pipefail

export ABRA_EXE="${ABRA_HOME}/target/debug/abra"

if ! has abra; then
   abra() {
      "$ABRA_EXE" "$@"
   }
   export -f abra
fi

_abra() {
   stty sane || true
   RUST_BACKTRACE=1 "$ABRA_EXE" "$@"
}
export -f _abra

_kill_tmux() {
   pkill -f tmux 2>/dev/null || true
}

_assert_tmux() {
   local -r log_file="$1"
   local -r sessions="$(tmux list-sessions)"
   if [ -z "$sessions" ]; then
      _kill_tmux
      cat "$log_file"
      return 1
   fi
}

_integration() {
   local -r log_file1="${ABRA_HOME}/target/ci1.log"
   local -r log_file2="${ABRA_HOME}/target/ci2.log"

   _kill_tmux || true
   rm "$log_file1" 2>/dev/null || true
   rm "$log_file2" 2>/dev/null || true

   echoerr "Starting sessions..."
   tmux new-session -d -s ci1 "abra rx --channel ci1 |& tee '${log_file1}'"
   tmux new-session -d -s ci2 "abra rx --channel ci2 --cmd 'echo \"lorem{}ipsum\"' |& tee '${log_file2}'"
   sleep 2

   echoerr "Sending commands..."
   _abra tx --channel ci1 --value "foo"
   echo "bar" | _abra tx --channel ci2
   sleep 1

   cat "$log_file1" | grep -q "foo" && cat "$log_file2" | grep -q "lorembaripsum"
}

test::set_suite "integration"
test::run "tmux" _integration

test::finish