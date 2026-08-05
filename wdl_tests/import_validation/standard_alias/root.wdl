version 1.3
import "lib.wdl" as lib
  alias Person as Patient
workflow root {
  Patient p = Patient { name: "Ann" }
  output {
    String out = p.name
  }
}
