version 1.3

task t {
  input {
    Int i
    String? suffix
  }
  command <<<
    echo ~{i}~{suffix}
  >>>
  output {
    String out = "ok"
  }
}

workflow ok {
  call t { i = 1, suffix = "x" }
}
