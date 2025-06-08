# enum-bitset

## Generate efficient bitsets out of your enum types.

![Crates.io Version](https://img.shields.io/crates/v/enum-bitset)
![docs.rs](https://img.shields.io/docsrs/enum-bitset)



The generated bitset type is much like the standard [HashSet] or [BTreeSet] types in that they contain a list of (
non-repeating) values of the base enum. But being implemented as a bitset, the memory usage is typically much lower than
the standard types, and all set operations (other than iteration) are constant time.

A bitset is a data structure that basically stores a list of bits, where each bit represents the presence or absence of
one of the possible values of the base enum type.

The main intended use-case of the [`EnumBitset`] macro is an enum that represents the possible `states` (in the general
meaning of the word) of some entity. It is common that, in some parts of your program, you need to so specify a set of
those states. For example,

* Filters:
    * The base enum represents the possible states of an entity. You want to present a list of entities to the user,
      allowing them to filter by one or more of those states.
    * Event subscriptions. What event types does every listener care about?

* Grouping / Hierarchy:
    * A large state machine is divided in phases. Every phase is a "sub-state-machine". You might have an enum
      representing all the states of the state machine, `State`; and another enum `Phase` representing the phases. You
      can apply [`EnumBitset`] to the `State` enum to automatically create a `StateSet` enum. Then you can store the
      states that belong to every phase in a `HashMap<Phase, StateSet>`.
    * In a graph, the set of nodes that can be reached from a given `Node` can be represented by a `NodeSet`.

* Non-exclusive values:
    * You have an enum `Mode` representing possible modes of operation of some part of some entity. But those modes are
      not mutually exclusive. You can store the modes that are active in the `ModeSet` type.
    * In a permission system, permissions might be represented by a `Permission` enum. Then, the set of permissions that
      a given user has can efficiently be represented by a `PermissionSet`.
    * What `Resources` are needed for a concrete task? You guess it: a `ResourceSet`!

## Basic usage

To use, add the `enum-bitset` crate as a dependency,

```bash
cargo add enum-bitset
```

or manually edit your `Cargo.toml` file

```toml
[dependencies]
enum-bitset = "0.1.0"
```

and add the derive [`EnumBitset`] macro on your enum

```rust
use enum_bitset::EnumBitset;

#[derive(EnumBitset, Copy, Clone)]
enum IpAddrKind {
    V4,
    V6,
}

let mut set = IpAddrKindSet::empty();
assert!(!set.contains(IpAddrKind::V4));
assert_eq!(set.len(), 0);

set.insert(IpAddrKind::V6);
assert!(!set.contains(IpAddrKind::V4));
assert!(set.contains(IpAddrKind::V6));

let set2 = IpAddrKind::V4 | IpAddrKind::V6;
assert!(set2.contains(IpAddrKind::V4));
assert!(set2.contains(IpAddrKind::V6));
assert!(!set2.is_empty());
assert!(set2.is_all());
assert_eq!(set2, IpAddrKindSet::all());

let set3 = set2 - IpAddrKind::V4;
assert!(set3.contains(IpAddrKind::V6));
assert!(!set3.contains(IpAddrKind::V4));
assert!(!set3.is_empty());
assert!(!set3.is_all());
assert_eq!(set3, IpAddrKind::V6.into());
```

You can check the generated type in the [example](example::ProgrammerStateSet) section. For more examples, see the
`tests` directory at [GitHub](https://github.com/glueball/enum-bitset/tree/main/tests).

## Configuration

The generated type macro can be configured using the `#[bitset]` attribute on the enum. As usual, the attribute must be
placed after the `derive` attribute and before the enum declaration.

The argument of the `bitset` attribute is a comma-separated list of key-value pairs. The following keys are supported:

Please, read the [documentation](https://docs.rs/enum_bitset) of the crate to find the full list of supported keys ans values.

## Alternatives
This crate exists because at work I had recurrent use cases where I needed to represent a set of values out of an enum.
While there are high-quality crates in the ecosystem that solve similar problems, after much experimentation I found
that my set of trade-offs and usability preferences were a bit different from the crates in the ecosystem.

Of course, this doesn't mean that existing crates are not good. Quite the opposite, being more mature and created by
more experienced developers, they are probably much better than this crate. I am publishing this crate hoping that it
might prove useful to someone else with similar preferences as me.

Here are some of the existing crates in the ecosystem.

### [enumset]

This is an incredibly good and feature complete crate, widely used. It supports enums with more than 128 variants.

My main concern with [enumset] is that the set type generated is generic over the base enum. That is, you use it as
`EnumSet<MyEnum>`, where the `EnumSet<T>` type lives in the `enumset` crate. That means that you cannot add inherent
methods to the generated type. This can be worked around by using an extension trait or a new-type wrapper, but it
turned out to be cumbersome for my use cases.

If that limitation is not a problem for you, then I *would recommend using [enumset]* as a very mature and high-quality
crate.

Note that [enumset] remains truthful to what a derive macro is originally meant to do: implement a trait on the type it
is applied to[^2]. In their case, they implement an `EnumSetType` trait, which is the requirement to use an enum as the
generic parameter of their `EnumSet` type.

[^2]: In contrast, our crate `enum-bitset` sort of abuses the derive macro to create a new type.
This kind of (ab)use is not uncommon in the Rust ecosystem.
Well, you could say it is a "derived type"... *right?*

### [bitfield](https://crates.io/crates/bitfield)

High-quality and well-known crate. However, its approach is very different: it allows generating structs that represent
bitfields, creating getters and setters to manipulate ranges of bits in the underlying value. Therefore, the generated
type does not have a relation with an enum.

### [bitflags](https://crates.io/crates/bitflags)

Another high-quality and well-known crate. However, its philosophy is different: instead of generating a set type
derived from an existing enum, it creates a structure with constants that represent a combination of flags. Its
documentation explicitly states that it is not intended to be used as a bitfield.

### [bitvec](https://crates.io/crates/bitvec)

Yet another high-quality and well-known crate. But once again, with a very different, lower-level, approach. It contains
types that work muck like std's collections of booleans, but using a one-bit-per-`bool ` approach.


[HashSet]: https://doc.rust-lang.org/std/collections/struct.HashSet.html

[BTreeSet]: https://doc.rust-lang.org/std/collections/struct.BTreeSet.html

[EnumDiscriminants]: https://docs.rs/strum/latest/strum/derive.EnumDiscriminants.html

[enumset]: https://crates.io/crates/enumset

