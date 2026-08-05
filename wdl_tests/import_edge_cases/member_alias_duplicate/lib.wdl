version 1.3

task first {
  command <<< echo first >>>
  output { String out = "first" }
}

task second {
  command <<< echo second >>>
  output { String out = "second" }
}
