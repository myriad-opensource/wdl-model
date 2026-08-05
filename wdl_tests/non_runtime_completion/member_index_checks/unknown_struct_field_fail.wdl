version 1.3

struct S {
  Int x
}

workflow bad {
  S s = S { x: 1 }
  Int y = s.missing
}
