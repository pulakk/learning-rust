# Notes
* Ownership means the `Power to destroy (Drop)`
* Ownership is important only for types stored in `heap` because `stack` doesn't have the issue of allocation and deallocation of memory in runtime
* A variable can have only one owner at a time
* `let x = y` moves the ownership of y to x and y is no longer usable, `let x = &y` or `let x = &mut y` only borrows the value of y to x, hence y still has ownership
* if y is a `String`, `let x = y` only does a shallow copy because it is on the heap. `y.clone()` can be used for deep-copy
* 2 `borrow` can overlap only if both of them are immutable `borrow`
* `String` type is `[ptr, length, capacity]` and `slice` or `string literals` is only `[ptr, length]`
* A `slice` can be borrowed from a string `s` using `&s[2..4]`