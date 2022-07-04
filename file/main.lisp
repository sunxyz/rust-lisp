(
    (load "./lib.lisp")
    (import (go) from "./async.lisp")
    (import (tcp-listener) from "./demo.lisp")
    (tcp-listener 8088)
)
