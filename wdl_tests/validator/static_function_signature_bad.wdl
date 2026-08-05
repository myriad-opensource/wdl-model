version 1.3

workflow bad {
  Array[String] k = keys([1])
  Array[Int] r = range("x")
  Array[Int] s = select_all(1)
  Int i = read_int(1)
  Boolean b = 1 && 2
  Int m = min("a", 1)
  Array[Int] t = transpose([1, 2])
  Boolean ck = contains_key([1], 1)
  Float sz = size("x", 1)
  Boolean ord = [1] < [2]
  Boolean c2 = contains("abc", 1)
  String bn = basename(1)
  Array[Array[Int]] ch = chunk([1, 2], "2")
  Array[Pair[Int, Int]] cr = cross(1, [1])
  File jp = join_paths(1, "a")
  File jp2 = join_paths("/tmp", 1)
  File jp3 = join_paths("/tmp", "a", 1)
  Int fl = floor("x")
  String su = sub("abc", 1, "z")
  String wm = write_map({"k": 1})
}
