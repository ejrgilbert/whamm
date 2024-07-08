(module
  (type (;0;) (func))
  (type (;1;) (func (param i32 i32) (result i32)))
  (func $foo (;0;) (type 0))
  (func (;1;) (type 0)
    call $foo
    call $foo
    call $foo
  )
  (memory (;0;) 1))