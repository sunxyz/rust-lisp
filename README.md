# rust-lisp

**lisp(Scheme) interpreter for rust**

## Implemented procedure (method)
* [x] number procedure (+ - * / < <= > >= = integer? float? number? integer=? float=? number=? float->integer float->string integer->float integer->string integer->char)
* [x] boolean operation (and or not)
* [x] byte byte-vector->string
* [x] char procedure (char? char=? char->number)
* [x] symbol procedure (symbol? symbol=? symbol->string)
* [x] string procedure (string? string=? mark-string string-length string-ref string-set! substring string-append string-copy string-find string-trim string-replace string->list ...)
* [x] cons procedure (cons car cdr set-car! set-cdr!)
* [x] list procedure (list list? list=? list-ref list-tail list-set! list_length append reverse list->vector list->string map for-each filter reduce)
* [x] vector procedure (vector? vector=? make-vector vector vector-length vector-ref vector-set! vector-fill! vector->list)
* [x] dict procedure (dict? dict=?  make-dict dict dict-length dict-get dict-rm! dict-put! dict-clear！ dict-contains-key? dict-keys->list dict-value->list dict->list)
* [x] quote '
* [x] procedure? method?
* [x] nil? 
* [x] type procedure (get-type-name is-type?)
* [x] define
* [x] let + let*
* [x] set
* [x] lambda support closure
* [x] branch (if do while) 
* [x] apply
* [x] define-macro support ` , ,@
* [x] display + newline
* [x] load support ; #
* [x] eval
* [x] lazy evaluation (delay promise? force)
* [x] io (file io | socket(net) io | console io) support (display newline call-with-tcp-listener call-with-input-file  call-with-output-file open-input-file  open-output-file input-port? output-port? port? read-char read-line read-string read-u8 read-byte-vector write-char write-string write-byte-vector write-u8)
* [x] concurrency (async await | thread ) support (thread-run join sleep make-lock lock-exp  make-barrier barrier-wait make-channel channel-send:(-> chan x) channel-done channel-recv:(<- chan) channel-for-each :(<-for-each fn chan) channel-map:(<-- fn chan)
* [ ] more can be implemented through macros 

## use
```
cargo build --release
```
```
cd ./target/release
```
**run**
```
./rust-lisp run ./aa.lisp 
```
**cmd**
```
./rust-lisp cmd 
```
or
```
./rust-lisp 
```
console 
```
--------------------------
 welcome rust-lisp v0.1.0 
-------------------------- 
 source code:https://github.com/sunxyz/rust-lisp 

> + 1 2 3 \
4 5 6
21
> :save
please input the file name:
./sample.lisp
 save file success
> call-with-input-file "./sample.lisp" read-string
(
    (+ 1 2 3 4 5 6)
) 
> load "./sample.lisp"
21
> :quit
 good bye!
```
## download rust-lisp

- [liunx_x86_rust-lisp](https://github.com/sunxyz/rust-lisp/blob/main/file/rust-lisp)

- [windows_x86_rust-lisp](https://github.com/sunxyz/rust-lisp/blob/main/file/rust-lisp.exe)

## lisp sample

**dialect**
* call-with-tcp-listener
```
(
    (define f (call-with-input-file "./demo.html" read-string))
    (call-with-tcp-listener "127.0.0.1:8088" ( lambda (in) (
       (display  (byte-vector->string (read-byte-vector in 4096)))
       (string-append "HTTP/1.1 200 OK\r\n\r\n" f)
    ))))
```
* thread + channel
```
(
    (define chan (make-channel))
    (define barrier (make-barrier 3))
    (define p (lambda (x) (
        (define i 0)
        (while (< i 10) (
            (display x i)
            (newline)
            (set! i (+ i 1))
        ))
        (barrier-wait barrier)
        (-> chan x)
        (channel-done chan)
    )))
    (thread-run p "你好,世界1！")
    (thread-run p "你好,世界2！")
    (thread-run p "你好,世界3！")
    (channel-for-each (lambda (x) (
        (display x)
        (newline)
    )) chan))
```
**web-sample**
* lib.lisp
```
(
    (define-macro def (lambda (name args . body) (`(define ,name (lambda ,args ,@body)))))
    (define-macro println (lambda (. args) (`((display ,@args) (newline) ))))
    (define-macro loop (lambda (. exp) (
        `(while (#t) ,exp)
    )))
    (define-macro export (lambda (. exports) (
       ` (dict (list ,@(map symbol->string exports)) (list ,@exports))
    )))
    (define-macro import (lambda (names form file)(
        (define export-info (load file))
        (`(,@(map (lambda (n) (`(define ,n (dict-get ,export-info (symbol->string ,n))))) names)))
    )))
    (define-macro ^ (lambda (. exp) (
       `(lambda () ,exp)
    )))
)
```
* async.lisp
```
(
    (define thread-qty (* (get-os-cpu-num) 2))
    (define barrier (make-barrier thread-qty))
    (define channel (make-channel))
    (do ((index 0 (+ 1 index)))
        ((= index thread-qty) nil) 
        (thread-run (lambda ()(
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
```
* web-sample.lisp
```
(
    (def handler (in out) (
        ;; (sleep 1)
        (println (current-thread-name))
        (display  (read-line in))
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
    (export tcp-listener)
)
```
* main.lisp

```
(
    (load "./lib.lisp")
    (import (go) from "./async.lisp")
    (import (tcp-listener) from "./web-sample.lisp")
    (tcp-listener 8088)
)
```
