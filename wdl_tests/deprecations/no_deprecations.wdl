version 1.3

task t {
  requirements {
    container: "ubuntu:latest"
    cpu: 1
  }
  command <<< echo hi >>>
  output {
    String out = "ok"
  }
}

workflow wf {
  call t
  output {
    String out = t.out
  }
}
