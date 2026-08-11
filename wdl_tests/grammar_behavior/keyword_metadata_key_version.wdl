version 1.2
task t {
  meta {
    version: "x"
  }
  command <<<
    echo "ok"
  >>>
}
workflow w {
  call t
}
