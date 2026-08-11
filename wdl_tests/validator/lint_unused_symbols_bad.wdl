version 1.3

task t {
  input {
    Int i
  }
  Int dead = 1
  command <<< echo ~{i} >>>
  output {
    Int out = i
  }
}

workflow bad {
  Int never_used = 1
  call t as c1 { input: i = 1 }
  scatter (x in [1, 2]) {
    Int y = 5
  }
}
