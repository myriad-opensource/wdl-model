version 1.3

task t {
  input {
    Int i
  }
  command <<< echo ~{i} >>>
  output {
    Int out = i
  }
}

workflow bad {
  input {
    MissingType x
  }
  call t as c1 { input: i = 1, i = 2 }
  call t as c2 after missing_call { input: i = 3 }
  call unknown_task as c3
}
