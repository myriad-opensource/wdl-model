version 1.3

struct S {
  Int x
}

task t {
  command <<< echo hi >>>
  output {
    String out = "ok"
  }
}

workflow ok {
  S s = S { x: 1 }
  Int x = s.x
  call t
  String z = t.out
  Int i = [1, 2][0]
}
