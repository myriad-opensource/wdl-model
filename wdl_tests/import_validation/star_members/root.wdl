version 1.3
import * from "star_lib.wdl"
import { selected_task as local_task } from "members_lib.wdl"
workflow root {
  call from_star
  call local_task { x = 7 }
  output {
    String a = from_star.out
    Int b = local_task.out
  }
}
