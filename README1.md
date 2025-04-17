# Getting Started in Rust

To get from zero to Hello, World in Rust we'll need to do a few things:
1. install rust (using rustup)
2. init a project (using cargo)
3. add your code
4. build, run, and test your code (using cargo)

## 1. rustup
The most common way to install and keep your Rust version up to date is to use rustup: https://rustup.rs/
Rustup will provide instructions to install Rust on your operating system.

Other avenues exist and are supported, but rustup provides a lot of utility like installing multiple versions of Rust, changing the active version, and installing tools that integrate with other Rust tools, like `cargo`.

## 2. Creating a Project
First, create a new repository in GitHub (personal or corporate, so long as the BreakMaster can access your submissions)

Once you have a repository, clone it locally and cd into the directory:
```sh
git clone github.rp-core.com/klogan/BreakPointV1
cd BreakPointV1
cargo init --bin
```

`cargo new BreakPointV1` can also be used to create a project (and a git repo), however I find it slightly more difficult to import a repo into GitHub than to create it in GitHub in the first place.

## 3. Adding your code
You should already have a working "Hello, World!" program at this point. You can build it with `cargo build`, run (and build) it using `cargo run`, and test it (there are no tests) with `cargo test`. When developing Rust it's important to get intimately familiar with cargo as it will be a centerpiece of the developer ecosystem.

Now, add the below code to the end of your `src/main.rs`:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_bytes() {
        let mut bytes = b"Hello, World!".to_vec();
        encode_bytes(&mut bytes);
        assert_eq!(bytes, b"Ifmmp-!Xpsme\"");
    }
}
```

This is how we create tests in Rust! It's very simple and tests are usually included alongside the testable code. `#[cfg(test)]` is an attribute that ensure the below code is only included in test build configurations, and `#[test]` is an attribute which marks a function as a test (so will be executed when running tests). `mod tests` could actually be named anything, while `use super::*` makes sure all definitions from the outer scope (which should just be `main` right now) are in-scope in our submodule `tests`.

This code demonstrates several key Rust syntax items, such as variable declaration, function calls, trait 'method' calls, macro calls, and declaring byte literals, and hopefully can be used to provide some hints about how to implement your own functions!

## 4. Building, Running, and Testing
As stated before, use cargo to build, run, and test the code.

Notice we added a test, `test_encode_bytes`, however `cargo build` still works! That's because the default build configuration is _not_ a test build, and so the test code _isn't even evaluated_!

If you run `cargo test` now, however, you'll notice it doesn't even compile!

Your challenge is twofold:
1. Get `cargo test` to compile successfully with no warnings
2. Get the tests themselves to pass!

Once you have what you believe is a working solution push up the solution and send the BreakMaster a link to the PR/branch/revision so it can be reviewed. When a tie-breaker is needed to determine which team provided a working solution first the GitHub changeset time will be used!

The BreakMaster will validate your code against the given test, and may include additional tests for additional edge cases so consider expanding your test coverage first!

# Relevant concepts
## Defining a function
creating a function in Rust is fairly straightfoward:
```rust
fn name(parameter1: Paramter1Type, parameter2: Parameter2Type) -> ReturnType {
    // code
}
```
A function may have no parameters (`fn name() -> ReturnType`) and may have no return value: `fn name()` or `fn name() -> ()`.

In Rust `()` is the unit type, a type with no value and no storage (similar to `void` in C-like languages, it represents the concept of nothing).

## Values vs References
In Rust everything is passed by value (moved, invalidating the original value) unless explicitly stated otherwise! This means in order to pass a reference to a value we must be explicit at both the call site _and_ the receiver side. The semantic is the same on both sides with `&`. Using `&` on a value creates a reference _to_ that value. Using `&` before a type makes the type a reference to that underlying type (`i32` is a 32 bit integer, which is a valid type, `&i32` is a reference to an `i32` which is itself also a valid type).

In the test we call `encode_bytes(&mut bytes);` which means the caller is explicitly passing a reference, so our receiver (the function) must accept a reference, this would not work:
```rust
fn encode_bytes(bytes: Vec<u8>) { ... }
```
A reference in Rust cannot be implicitly converted to a value like in other languages. In most languages that would involve either creating a copy-on-write reference or creating a deep copy of the object. Rust doesn't do this, if you want a copy you must be explicit:
```rust
let x = "Hello, World!".to_string();
let y = x.clone(); // here we explicitly copy the value
let z = &x; // here we explicitly reference (borrow) the value of x
let broken = x; // this _takes_ the value of x, which isn't allowed as z is still borrowing x
```

## Mutability
Everything in Rust is immutible by default! The function we need, `encode_bytes`, is required to change the value of the parameter. This is only possible by explicitly marking a variable as mutable:
```rust
fn encode_bytes(bytes: &mut Vec<u8>)
```

Here we must explicitly state that `bytes` is both a borrow (with `&`) and the borrowed-type `Vec<u8>` is mutable. If you remove `mut` your borrow will be a shared (immutible) borrow, and you will not be permitted to make any changes to the borrowed value.

Now with a mutable borrow we can change values in the vector!
```rust
if bytes.len() > 0 {
    bytes[0] = 'H';
}
```

## Iteration and for loops
In order to change all the values of the vector in `encode_bytes` we'll need to loop over them. There are many ways to do this in Rust.

There's the standard index-based for loop syntax:
```rust
for i in 0..bytes.len() {
    bytes[i] = 0;
}
```

And there is the approach using iterators:
```rust
for value in bytes.iter() {
    println!("value: {value}");
}
```

But notably, in that above example we cannot modify the values anymore! You might be tempted to change `value` to mut:
```rust
for mut value in bytes.iter() {
    value = 0;
}
```
But this is not doing what you might expect! This is more akin to writing `mut &u8` than `&mut u8` (the latter being the correct syntax we want). This has made our reference mutable, not the value that is referenced! We can change the definition to get what we want:
```rust
for &mut value in bytes.iter() {
    value = 0;
}
```
But this _also_ doesn't do what we want! First we're trying to create a mutable reference from an immutible reference (it turns out `iter` is returning an iterator of _shared_ references), but also we've actually used Rust pattern matching to _remove_ the reference, `value` is a ... value! Removing the `&mut` while make value what it is supposed to be, `&u8`, but that isn't mutable and we're back to square one!

The answer is change _how_ we're iterating, `iter` will always return an immutible view:
```rust
for value in bytes.iter_mut() {
    *value = 0;
}
```
Now `value` is a `&mut u8` like we want, meaning a reference to a mutible byte! Note we had to dereference now, and it wasn't simply `value = 0` but rather `*value = 0` as we're assigning the referenced value, not assigning the reference itself.

## Conclusion
These tools should be enough to get you going, but remember if you get Stuck you can always ask the BreakMaster for help!