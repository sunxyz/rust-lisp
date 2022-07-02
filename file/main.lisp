(
    (load "./lib.lisp")
    ;; (import (tcp-listener) from "./demo.lisp")
    ;; (println  tcp-listener)
    ;; (tcp-listener 8088)
    (def req-read-string  (in) (byte-vector->string (read-byte-vector in 128)))
    (def handler (in out) (
        ;; (sleep 1)
        (display  (req-read-string in))
        (write-string "HTTP/1.1 200 OK\r\n\r\n hello word" out )
    ))
    ;; (call-with-tcp-listener "127.0.0.1:8088" ( lambda (in out) (
    ;;      (thread-run (lambda () (handler in out)))
    ;; )))
    (call-with-tcp-listener "127.0.0.1:8088" ( lambda (in out) (
         ( handler in out)
    )))
)
