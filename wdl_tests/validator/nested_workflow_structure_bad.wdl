version 1.3

task t {
  input {
    Int i
  }
  command <<< echo ~{i} >>>
  output {
    Int out = i
  }
}

workflow bad {
  scatter (x in [1, 2]) {
    Int a = 1
    Int a = 2
    call t as c2 after c3 { input: i = x }
    call t as c3 { input: i = x }
  }
}
