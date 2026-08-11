version 1.2
task find {
  input {
    String in
  }
  command <<<
    echo ~{in}
  >>>
}
workflow w {
  call find
}
