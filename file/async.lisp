(
    (define thread-qty (* (get-os-cpu-num) 2))
    (define thread-count 1)
    (define barrier (make-barrier thread-qty))
    (define channel (make-channel))
    (while (<  thread-count thread-qty)
        (  
            (thread-run (^(
                (channel-for-each (lambda (task) (
                    (task)
                )) channel)
                (barrier-wait barrier)
            )))
            (set! thread-count (+ thread-count 1))))
    (def go (task) (
      -> channel task
    ))
    (export go)
)