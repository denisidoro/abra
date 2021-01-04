_abra_hook() {
  echo foo
  trap -- '' SIGINT;
  abra tx --channel pwd --value "$PWD" &> /dev/null || true;
  trap - SIGINT;
  echo bar
}

# typeset -ag precmd_functions;
# if [[ -z ${precmd_functions[(r)_abra_hook]} ]]; then
#   precmd_functions=( _abra_hook ${precmd_functions[@]} )
# fi

typeset -ag chpwd_functions;
if [[ -z ${chpwd_functions[(r)_abra_hook]} ]]; then
  chpwd_functions=( _abra_hook ${chpwd_functions[@]} )
fi