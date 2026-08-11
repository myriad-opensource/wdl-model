version 1.3

import { t as local_task } from "lib.wdl"

task local_task {
  command <<< echo local >>>
  output { String out = "local" }
}

workflow root {}
