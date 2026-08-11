version 1.3

task t {
  command <<< echo hi >>>
  output { String out = "ok" }
}
