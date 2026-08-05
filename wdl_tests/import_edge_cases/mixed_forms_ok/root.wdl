version 1.3

import "lib.wdl" as lib
  alias Shared as LocalShared
import * from "star.wdl"
import { selected_task as local_task } from "members.wdl"

workflow root {
  call lib.ns_task
  call star_task
  call local_task { x = 7 }
  LocalShared s = LocalShared { value: "ok" }
  output {
    String a = lib.ns_task.out
    String b = star_task.out
    Int c = local_task.out
    String d = s.value
  }
}
