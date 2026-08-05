version 1.3
import "lib.wdl" as lib
import "types.wdl"
  alias Person as Patient
  alias Status as ImportStatus
import * from "star.wdl"
import { selected_task as local_task, selected_flow as local_flow, SelectedStruct as LocalStruct, SelectedEnum as LocalEnum } from "members.wdl"
workflow root {}
