version 1.3

task t {
  command <<< echo imported >>>
  output { String out = "imported" }
}
