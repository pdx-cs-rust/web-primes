# web-primes: simple functional web service
Bart Massey 2021

This simple web service uses the
[Tide](https://crates.io/crates/tide) web framework to
provide a server that supplies random prime numbers with a
specified length in bits. **This webapp is not guaranteed to
produce good primes, or even primes, or even not just
crash. Don't use it for security purposes.**

To try it out, `cargo run --release` and then browse to
`http://127.0.0.1:8080`.

Future work is to speed up large prime generation by taking
better advantage of the
[`glass_pumpkin`](https://crates.io/crates/glass_pumpkin/0.2.1)
crate's facilities.

This work is provided under the "MIT License". Please see
the file `LICENSE.txt` in this distro for license terms.

This webapp took me about two hours to write, debug and
deploy. Most of that time was spent learning the Tide API
(about 1 hour) and implementing the prime generator for
large primes (about half an hour).
