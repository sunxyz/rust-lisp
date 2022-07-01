(
    (define-macro def (lambda (name args . body) (`(define ,name (lambda ,args ,@body)))))
    (define-macro println (lambda (. args) (`((display ,@args) (newline) ))))
    (define-macro define-record-type (lambda (name . args) (
       (define var-names (cdr (car args)))
       (define len ((list-length var-names)))
       (define _index 0)
       (define indexs (map (lambda () ((define v _index) (set! _index (+ _index 1)) v)) var-names))
       (`(
            (define (string->symbol (string-append "make-" (symbol->string ,name))) (lambda ,var-names (
                (define index 0)
                (define vec (make-vector ,len 0) )
                (for-each (lambda (v) (
                    (vector-set! vec index v)
                    (set! index (+ index 1))
                    )) (list ,@var-names))
                (vector (symbol->string ,name) vec)    
            )))
            (define (string->symbol (string-append (symbol->string ,name) "?" )) (lambda (v) (
                string=? (vector-ref v 0) ((symbol->string ,name))
            )))
            (,@(map  (lambda (x index) (
             `(define (string->symbol (string-append (symbol->string ,name) "-"  (symbol->string ,x) )) (lambda (r) (vector-ref(vector-ref r 1) ,index)))
            )) (` ,@var-names) indexs))
            (,@(map  (lambda (x index) (
             `(define (string->symbol (string-append (symbol->string ,name) "-"  (symbol->string ,x) "-set!")) (lambda (r v) (vector-set! (vector-ref r 1) ,index v)))
            )) (` ,@var-names) indexs))
        ))

    )))
    (define-macro loop (lambda (. exp) (
        `(while (#t) ,exp)
    )))
    (define-macro export(lambda (. exports) (
       ` (dict (list ,@(map symbol->string exports)) (list ,@exports))
    )))
    (define-macro import(lambda (names form file)(
        (define export-info (load file))
        (`(,@(map (lambda (n) (`(define ,n (dict-get ,export-info (symbol->string ,n))))) names)))
    )))
)