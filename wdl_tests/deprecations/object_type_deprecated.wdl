version 1.3

workflow wf {
  Object? data = None
  Boolean seen = defined(data)
  output {
    Boolean out = seen
  }
}
