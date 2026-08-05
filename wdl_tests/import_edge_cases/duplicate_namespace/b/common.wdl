version 1.3

task b_task {
  command <<< echo b >>>
  output { String out = "b" }
}
