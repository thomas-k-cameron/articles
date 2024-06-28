# Optimizing Serde or Rust lang's match statement for compile time known strings

Here are two functions. 
One matches against 
These two functions are identical, and it pretty much does the same thing.
However, `bytes_match` is more likely to be faster when you measure the performance.

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


I created a benchmark with 1,000 match arm to see the difference and here is the result.

## So why is this the case?  

When Rust checks the equality of a `String`, it checks whether each and every byte before it finally decides whether the two value is equal or not.

However, when Rust checks the equality of bytes, Rust will conclude that two values aren't equal as soon as it finds a mismatching byte.

Let's take a look at the CFG of generated assembly code.

```asm

```

## Does it matter?

I think the performance difference is negligible in many cases but I think it matters.

Rust is a language that has a lots of compiled-time known string thanks to `serde`.
Serde generates a match statement which a match arm to parse the value for each field.  

I think it is more likely that Rust's string comparison will evaluate to false.

Although Rust will not compare the content of string unless it has the same length it will add up.
