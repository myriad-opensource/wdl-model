version 1.3

task selected_task {
  input { Int x }
  command <<< echo ~{x} >>>
  output { Int out = x }
}
