version 1.3

import "lib.wdl" as dup

task dup {
  command <<< echo local >>>
  output { String out = "local" }
}

workflow root {}
