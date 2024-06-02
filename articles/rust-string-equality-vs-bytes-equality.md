# Optimizing Serde or Rust lang's match statement for compile time known strings

## Benchmark

Take a look at an example below.
These are two identical function, and it pretty much does the same thing.

However, `bytes_match` will be faster when you take a benchmark.

```rust

fn str_match(s: &str) -> i64
{
    match s {
        "match-arm-1" => 1
        "meoww-dog-2" => 2
        // lots of uuid-s
        _ => -1
    }
}

fn bytes_match(s: &str) -> i64
{
    match s.as_bytes() {
        b"match-arm-1" => 1
        b"meoww-dog-2" => 2
        // lots of uuid-s
        _ => -1
    }
}
```

## So why is this the case?  

Rust's string comparison is quite different from C lang's `strcmp`.
Let's take a look at CFG of generated assembly code.
e.g.

```asm

```

- Clang checks byte-by-byte but Rust is byte-packed
- Rust tries to load data from memory on every attempt to do an equality check.
- Clang breaks as soon as it finds a un-equal byte but Rust doesn't break until it finish checking every byte

By the way, Rust's `[u8]`'s equality check's implementation is the same as `strcmnp`.
e.g.

```asm

```


## So rust has lots of compile-time known strings

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

## Is Clang implementation superior?

Let's take a benchmark.  

If C lang's implementation is

I have
