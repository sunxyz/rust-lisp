# rust-lisp

**lisp(Scheme) interpreter for rust**

## Implemented procedure (method)
* [x] number procedure (+ - * / < <= > >= = number? number=? number->string number->char)
* [x] boolean operation (and or not)
* [x] char procedure (char? char=? char->number)
* [x] symbol procedure (symbol? symbol=? symbol->string)
* [x] string procedure (string? string=? mark-string string-length string-ref string-set! substring string-append string-copy string-find string-trim string-replace string->list ...)
* [x] cons procedure (cons car cdr set-car! set-cdr!)
* [x] list procedure (list list? list=? list-ref list-tail list-set! list_length append reverse list->vector list->string map for-each filter reduce)
* [x] vector procedure (vector? vector=? make-vector vector vector-length vector-ref vector-set! vector-fill! vector->list)
* [x] quote '
* [x] procedure? method?
* [x] nil? 
* [x] type procedure (get-type-name is-type?)
* [x] define
* [x] let + let*
* [x] set
* [x] lambda support closure
* [x] branch (if)
* [x] apply
* [x] define-macro support ` , ,@
* [x] display + newline
* [x] load support ; #
* [x] eval
* [x] lazy evaluation (delay promise? force)
* [x] io (file io | socket(net) io | console io)
* [ ] concurrency (async await | thread )
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