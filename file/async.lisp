(
    (define thread-qty (* (get-os-cpu-num) 2))
    (define thread-count 1)
    (define barrier (make-barrier thread-qty))
    (define channel (make-channel))
    (while (<  thread-count thread-qty)
        (  
            (thread-run (lambda () (
                (channel-for-each (lambda (task) (
                    ( task)
                )) channel)
                (barrier-wait barrier)
                ;; (loop ((<- channel)))
            )))
            (set! thread-count (+ thread-count 1))))
    (define go (lambda-lep (task) (
       (apply (`(-> ,channel ,task)))
    )))
    (export go)
)