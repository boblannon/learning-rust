# Journal

A place to write down things I learn.

## You can't sort a `Vec` of floats

Floats don't have the `Ord` trait, only `PartialOrd`. A quick workaround is:

```rust
v.sort_by(|a, b| a.partial_cmp(b).unwrap());
```

The detailed reason why is that
[IEEE 754](https://en.wikipedia.org/wiki/IEEE_754)'s `totalOrdering` predicate
prescribes behavior for `-NaN`, which isn't defined in Rust. Here's an
approximate implementation of IEEE 754 totalOrdering:
https://users.rust-lang.org/t/how-to-sort-a-vec-of-floats/2838/3.

[Another point of view:](https://stackoverflow.com/a/26490185) "IEEE 754's
definition of the ordinary comparison operators `<=`, `>=`, ... makes them very
useful in general, if not when you need a total order. In particular, it is
easy to write conditions so that NaN inputs will be sent to the error branch:"

```javascript
if (!(x <= MAX)) { // NaN makes this condition true
  error();
}

if (!(x >= MIN)) { // NaN makes this condition true
  error();
}
```

Because `>=` and `<=` are so useful, they are the operations implemented as
single, fast instructions in modern processors. The totalOrder predicate from
IEEE 754 is typically not implemented in hardware. Programming languages map
the fast instructions to constructs in the language and leave anyone who
exceptionally needs totalOrder to pick it from a library or even to define it
themselves.

**Why I hate it:**
- I just want to `.sort()`!
**Why I shouldn't:**
- In other languages, you can find non-deterministic float NaN sorting
  ```python
  >>> nan = float('nan')
  >>> xs = [1., nan, 0., 3., 5., 2.]
  >>> sorted(xs)
  [1.0, nan, 0.0, 2.0, 3.0, 5.0]
  >>> xs = [1., 0., 3., nan, 5., 2.]
  >>> sorted(xs)
  [0.0, 1.0, 2.0, 3.0, nan, 5.0]
  ```
- These errors can happen at runtime and be difficult to debug


## usize and isize

@apendleton:

> You can't index into vecs with anything other than a usize, because while on
> 64-bit platforms, pointers are 64 bits, on 32-bit platforms (or smaller),
> there's the potential for an overflow. The backstory is that C/C++ don't make
> this kind of distinction or at least hasn't historically, so there are just
> types long `int`, `long in`, etc., but they're not always guaranteed to be
> the same on all platforms (so like, an int might be 16 bits on one platform
> and 32 on another, etc.), meaning code that works on one platform might
> silently not another in a difficult to debug way. Newer C/C++ iterations have
> added types with names that explicit indicate the size (so like, `uint64_t`
> is the unsigned 64-bit int type), and Rust pushed hard in that direction for
> most int types, so mostly people use `u64` or `i32` or whatever, but the
> catch is that you really do need it to be platform-specific for pointers

## Custom Errors

Minimal error definition: http://stevedonovan.github.io/rust-gentle-intro/6-error-handling.html

Minimal error enum definition: https://medium.com/@fredrikanderzon/custom-error-types-in-rust-and-the-operator-b499d0fb2925

## Why you can't use `let` outside of a block

this is sortof navel-gazey but i was searching for the right terminology
associated with a concept.  i’m asking myself “why can’t i have `let`
statements in the outermost scope of a module?” and i know that the functional
answer is “you can’t have `let` statements outside of a function,” or, more
generally, that the outermost scope is reserved for definitions (of functions,
structs, enums, etc) but i’m curious as to whether there’s a more pithy way to
describe that.

@ingalls:

> You can't use `let` variables outside of a block scope as let allows for
> potential mutability which while memory safe, is not thread safe. `const` and
> `static` can be used in a global context as since they promise non-mutability
> and thread-locality. Side note: I've also seen a couple folks use
> `thread_local!` combined with a `RefCel` to get a safe globalish variable
> (within a given thread)

## Listing only some fields when destructuring

Here's a snippet:

```rust
let mut total_edit_distance = 0;
for word in words {
    match word {
        &&Word::Full{ ref string,  ref id, ref edit_distance } => {
            total_edit_distance += *edit_distance;
        },
        _ => (),
    }
}
```

but this gives you a compiler warning:

```rust
warning: unused variable: `string`
  --> src/phrase/word.rs:46:31
   |
46 |                 &&Word::Full{ ref string,  ref id, ref edit_distance } => {
   |                               ^^^^^^^^^^ help: consider using `_string` instead

warning: unused variable: `id`
  --> src/phrase/word.rs:46:44
   |
46 |                 &&Word::Full{ ref string,  ref id, ref edit_distance } => {
   |                                            ^^^^^^ help: consider using `_id` instead
```

Tried removing `string` and `id`. No good, error: `pattern does not mention field string`

Tried renaming (as it suggests). This did work, but looks ugly:

```rust
&&Word::Full{ string: ref _string,  id: ref _id, ref edit_distance } => {
```

Turns out range operator can help here:

```rust
&&Word::Full{ ref edit_distance, .. } => {
```


