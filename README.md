Different options to implement something similar to polymorphism in an ergonomic
way.

Rust offer 2 main options:
- dyn Trait: difficult to get the concrete type if needed
- enums: becomes too verbose as you need to match all the enum variants all the time

In this repo, options 1 and 2 uses `dyn Trait`, showing different options to
recover the concrete type from the `dyn Trait` object. Option 3 uses the crate
enum_dispatch to allow using enums in a less verbose way.

Test with `cargo run --bin optionN`
