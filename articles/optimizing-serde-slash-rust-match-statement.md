# Optimizing Serde or Rust lang's match statement for compile time known strings

## So rust has lots of compile-time known strings...

Thanks to `serde` crate, Rust can have a lot of comparison of compile-time known strings.

e.g.

```rust
match some_str {
    "thing" => 1,
    "stuff" => 2,
    "these" => 3,
    _ => -1 // lol
}
```

Under the hood, Rust compiles this into a large if-else ladder, where it loads pieces of data everytime a block is executed.  

Modern CPU has big CPU cache and these memory loads should be cached and the performance impact might be negligible.  
However, these can affect the performance significantly, especially on micro-controlers and less-powerful chips.

Even on powerful CPUs I think it can start to make a difference on memory-heavy applications.

## Clang `strcmp` vs Rust `String::Eq`

Rust's string comparison is quite different from C lang's `strcmp`.
Let's take a look at CFG of generated assembly code.
e.g.

```asm

```

> [!NOTE]  
> For your information, Rust's `[u8]`'s equality check's implementation is the same as `strcmnp`.


- Clang checks byte-by-byte but Rust is byte-packed
- Rust tries to load data from memory on every attempt to do an equality check.
- Clang breaks as soon as it finds a un-equal byte but Rust doesn't break until it finish checking every byte



e.g.

```asm

```

## Is Clang implementation superior?

There are few things to consider.

If C lang's implementation is 

I have 