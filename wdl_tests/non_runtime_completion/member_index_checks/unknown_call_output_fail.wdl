version 1.3

task t {
  command <<< echo hi >>>
  output {
    String out = "ok"
  }
}

workflow bad {
  call t
  String z = t.missing
}
