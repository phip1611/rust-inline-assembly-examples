# Reading and Storing Values from/in CPU registers in Rust (Inline Assembly)

This repository contains a few basic examples about working with general purpose registers in Rust
via `inline assembly`. It shows examples for the deprecated `llvm_asm!`-macro and the **newer** `asm!`-macro [0]. 
See Code for comments.

## Interesting Links
[0] https://blog.rust-lang.org/inside-rust/2020/06/08/new-inline-asm.html \
[1] https://en.wikipedia.org/wiki/X86_assembly_language#Syntax \
[2] https://doc.rust-lang.org/nightly/unstable-book/library-features/llvm-asm.html (old asm macro) \
[3] https://doc.rust-lang.org/nightly/unstable-book/library-features/asm.html (new asm macro) 
