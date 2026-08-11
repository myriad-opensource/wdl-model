version 1.3

task a_task {
  command <<< echo a >>>
  output { String out = "a" }
}
