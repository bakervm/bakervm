# Reserved memory
In general the first 20 slots of the value-index are reserved by the system. That that the user value-index begins at `$vi(20)`. Space in the user value-index has to be allocated. To allocate space in the user value-index, you have to advance the base pointer `$bp` by adding an address to it. So by writing:
```
push $st, @7
add $bp, $st
```
you are allocating 7 safe slots in the user value-index. Allocation should be already done, when using the value-index. Otherwise you risk corrupting the memory. So in that case you would be able to use `$vi(20)` to `$vi(26)`.


| index address | data type | usage                                                  |
|--------------:|:---------:|--------------------------------------------------------|
|             0 |  Address  | framebuffer cursor: Set by writing an address to vi(0) |
|             1 |  Address  | display width                                          |
|             2 |  Address  | display height                                         |
|             3 |  Address  | mouse x                                                |
|             4 |  Address  | mouse y                                                |
|             5 |  Boolean  | left mouse button                                      |
|             6 |  Boolean  | middle mouse button                                    |
|             7 |  Boolean  | right mouse button                                     |
