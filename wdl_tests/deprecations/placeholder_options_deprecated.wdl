version 1.3

task t {
  input {
    Array[String] xs
  }
  command <<<
    echo ~{sep=", " xs}
  >>>
  output {
    String out = "ok"
  }
}

workflow wf {
  call t { xs = ["a", "b"] }
}
