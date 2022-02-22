# Count macro
A macro to allow for compile time counting

# How to use this

Every instance of `_int_` will be replaced with either a literal or an ident.

`count_macro::count` will panic in debug mode if counter exceeds usize.

If you wish to wrap to 0, please use `count_macro::wrapping_count`.

## Examples

### Ident to literal
```rust
use count_macro::count;

let a = count!(vec![_int_, _int_, _int_]);
assert_eq!(a, vec![0, 1, 2]);

```

### Ident to ident
```rust
use count_macro::count;

count! {
    let a_int_ = "Hello";
    let a_int_ = "World";
}

assert_eq!(a0, "Hello");
assert_eq!(a1, "World");

```

### In macro
```rust
use count_macro::count;

macro_rules! my_macro {
    ($($v:expr),*) => {
        count!{
            $(
                let _ = $v; // Ignoring $v

                println!("{}", _int_);
            )*
        }
    };
}

my_macro!('@', '@', '@', '@'); // Will print from 0 to 3

```

### Multiple counters

with `_int_countername_` you'll be able to create a new counter called "countername".

This won't be incremented by `_int_` or any other counter such as `_int_0_` or `_int_x_`.

```rust
use count_macro::count;

// With two different counters
// _int_ does not increment _int_name_
count! {
    let a_int_ = _int_name_;
    let a_int_ = _int_name_;
    let a_int_ = _int_name_;
}

assert_eq!(a0, 0);
assert_eq!(a1, 1);
assert_eq!(a2, 2);

```
