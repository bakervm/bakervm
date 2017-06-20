# BASM naming conventions

## Guards
Usually when writing a library you should enclose your library file with a guard:
```
jmp _my_library_namespace_and_method_name

  ; Your code goes here

._my_library_namespace_and_method_name
```

## Modules and functions
Since dots are allowed in label names, you should always namespaces in your library:
```
jmp _my_library_namespace_and_method_name

.my.library.namespace.and.method_name
  ; Your function code ...
ret

._my_library_namespace_and_method_name
```

## Member functions and jumps
Member functions and jumps should always be prefixed with the fully qualified module and function name and have a spacer of two underscores:
```
jmp _my_library_namespace_and_method_name

.my.library.namespace.and.method_name
  ; Your function code ...

  .my.library.namespace.and.method_name__some_internal_jump


  .my.library.namespace.and.method_name__some_internal_function
    ; Code ...
  ret
ret

._my_library_namespace_and_method_name
```
