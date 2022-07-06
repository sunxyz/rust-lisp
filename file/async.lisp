(
    (define thread-qty (* (get-os-cpu-num) 2))
    (define barrier (make-barrier thread-qty))
    (define channel (make-channel))
    (do ((index 0 (+ 1 index)))
        ((= index thread-qty) nil) 
        (thread-run (^(
            (channel-for-each (lambda (task) (
                (task)
            )) channel)
            (barrier-wait barrier)
            ))))
    (def go (task) (
      -> task channel
    ))
    (export go)
)