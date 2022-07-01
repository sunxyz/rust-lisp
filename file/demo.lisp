(
    
    (def tcp-listener (port) (
        (println "tcp-listener: port: " port)
        (def req-read-string  (in) (byte-vector->string (read-byte-vector in 4096)))
        (call-with-tcp-listener (string-append "127.0.0.1:" port) ( lambda (in) (
            (display  (req-read-string in))
            "HTTP/1.1 200 OK\r\n\r\n hello word")))))
    (export tcp-listener)
)