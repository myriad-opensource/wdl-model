version 1.3

task t {
  input {
    Int x
  }
  command <<< echo ~{x} >>>
  output {
    Int out = x
  }
}

workflow ok {
  Int i = 1
  call t { x = i }
  output {
    Int y = t.out
  }
}
