version 1.3

task t {
  runtime {
    docker: "ubuntu:latest"
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
