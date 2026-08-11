version 1.1

task t {
  meta {
    author: "alice"
  }

  parameter_meta {
    x: "number"
  }

  input {
    Int x
  }

  command {
    echo hi
  }

  output {
    String out = read_string(stdout())
  }
}
