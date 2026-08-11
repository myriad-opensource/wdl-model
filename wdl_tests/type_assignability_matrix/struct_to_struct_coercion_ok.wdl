version 1.3

struct A {
  String s
}

struct B {
  A child
  Int i
}

struct C {
  String s
}

struct D {
  C child
  Int i
}

workflow struct_to_struct_coercion_ok {
  B source = B {
    child: A { s: "hello" },
    i: 42
  }

  D coerced = source
}
