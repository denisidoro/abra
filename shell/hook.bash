_abra_hook() {
  local previous_exit_status=$?;
  trap -- '' SIGINT;
  abra tx --channel pwd --value "$PWD" &> /dev/null || true;
  trap - SIGINT;
  return $previous_exit_status;
};

if ! [[ "${PROMPT_COMMAND:-}" =~ _abra_hook ]]; then
  PROMPT_COMMAND="_abra_hook${PROMPT_COMMAND:+;$PROMPT_COMMAND}"
fi
