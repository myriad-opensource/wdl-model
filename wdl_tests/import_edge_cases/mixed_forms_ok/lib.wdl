version 1.3

task ns_task {
  command <<< echo ns >>>
  output { String out = "ns" }
}

struct Shared {
  String value
}
