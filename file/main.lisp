(
    (load "./lib.lisp")
    ;; (import (tcp-listener) from "./demo.lisp")
    ;; (println  tcp-listener)
    ;; (tcp-listener 8088)
    (def req-read-string  (in) (byte-vector->string (read-byte-vector in 4096)))
    (call-with-tcp-listener "127.0.0.1:8088" ( lambda (in) (
        (display  (req-read-string in))
        "HTTP/1.1 200 OK\r\n\r\n hello word")))
)