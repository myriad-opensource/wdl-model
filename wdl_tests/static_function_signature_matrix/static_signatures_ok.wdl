version 1.3

workflow ok {
  Array[String] k = keys({"a": 1})
  Array[Int] r = range(3)
  Boolean c1 = contains([1, 2, 3], 2)
  Boolean c2 = contains("abc", "b")
  Array[Array[Int]] ch = chunk([1, 2, 3], 2)
  Array[Pair[Int, String]] cr = cross([1, 2], ["x", "y"])
  File p1 = join_paths("/tmp", "a")
  File p2 = join_paths("/tmp", "a", "b")
  String b = basename("/tmp/a.txt", ".txt")
  Float s = size("/tmp/a.txt", "MB")
}
