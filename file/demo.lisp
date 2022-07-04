(
    (def req-read-string  (in) (
        byte-vector->string (read-byte-vector in 128)
    ))
    (def handler (in out) (
        ;; (sleep 1)
        (println (current-thread-name))
        (display  (req-read-string in))
        (write-string "HTTP/1.1 200 OK\r\n\r\n hello word" out )
    ))
    (def tcp-listener (port) (
        (println "tcp-listener: port: " port)
        (call-with-tcp-listener (string-append "127.0.0.1:" port) ( lambda (in out) (
            (go (lambda () (
                handler in out
            )))
        )))
    ))

    ;; (import (tcp-listener) from "./demo.lisp")
    ;; (println  tcp-listener)
    ;; (tcp-listener 8088)
    ;; (def req-read-string  (in) (byte-vector->string (read-byte-vector in 128)))
    ;; (def handler (in out) (
    ;;     ;; (sleep 1)
    ;;     (println (current-thread-name))
    ;;     (display  (req-read-string in))
    ;;     (write-string "HTTP/1.1 200 OK\r\n\r\n hello word" out )
    ;; ))
    ;; ;; sample 1
    ;; ;; (call-with-tcp-listener "127.0.0.1:8088" ( lambda (in out) (
    ;; ;;      (thread-run (lambda () (handler in out)))
    ;; ;; )))
    ;; ;; sample 2
    ;; ;; (call-with-tcp-listener "127.0.0.1:8088" ( lambda (in out) (
    ;; ;;      ( handler in out)
    ;; ;; )))
    ;; ;; sample 3
    ;; ;; (call-with-tcp-listener "127.0.0.1:8088" ( lambda (in) (
    ;; ;;      (display  (req-read-string in))
    ;; ;;      "HTTP/1.1 200 OK\r\n\r\n hello word"
    ;; ;; )))
    ;; (import (go) from "./async.lisp")
    ;; ;; (define i 0)
    ;; ;; (while (< i 100)
    ;; ;;    (go (lambda () (println (current-thread-name) ":=>" i)))
    ;; ;;    (set! i (+ i 1))
    ;; ;; )
    ;; ;; (sleep 1000)
    ;; ;;  sample 4
    ;; (call-with-tcp-listener "127.0.0.1:8088" ( lambda (in out) (
    ;;     (go (lambda () (handler in out)))
    ;; )))
    (export tcp-listener)
)