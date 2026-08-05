version 1.3

import "lib.wdl"
  alias Address as Addr
  alias Person as PersonAlias

workflow root {
  PersonAlias p = PersonAlias { addr: Addr { city: "X" } }
  output {
    String city = p.addr.city
  }
}
