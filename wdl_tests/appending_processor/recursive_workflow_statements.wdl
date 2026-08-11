version 1.1

task t {
  input {
    Int i
  }

  command {
    echo "${i}"
  }

  output {
    String out = read_string(stdout())
  }
}

workflow nested {
  Int x = 1
  if (x == 1) {
    call t { input: i = x }
  } else {
    scatter (n in [1, 2]) {
      Int y = n
    }
  }
}
