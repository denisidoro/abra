rxout() { 
   abra rx --channel "${1}_out"   
}

rxerr() { 
   abra rx --channel "${1}_err"   
}

txspl() { 
   local -r  channel="$1"
   abra rx --channel test_out # window 1
   abra rx --channel test_err # window 2
   "$@" >(abra tx --channel "${channel}_out") 2> >(abra tx --channel "${channel}_err") 
}

rxls() {
   abra rx --channel pwd --cmd 'ls {}'
}

faketty() {
   abra faketty --cmd "$*"
}

### USAGE

# rxout test # window 1
# rxerr err # window 2
# txspl cargo test # window 3

# faketty ls .