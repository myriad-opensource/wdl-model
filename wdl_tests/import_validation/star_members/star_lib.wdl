version 1.3
task from_star {
  command <<< echo hi >>>
  output { String out = "ok" }
}
