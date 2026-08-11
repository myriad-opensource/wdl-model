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
  Array[Int] xs = [1, 2]
  Int first = xs[0]
  call t { x = i }
  output {
    Int y = t.out
  }
}
