version 1.3

task t {
  requirements {
    container: "ubuntu:latest"
    cpu: 1
    memory: "2 GiB"
  }
  hints {
    max_cpu: 2
    short_task: true
  }
  command <<< echo hi >>>
  output {
    String out = "ok"
  }
}

workflow ok {
  call t
  output {
    String out = t.out
  }
}
