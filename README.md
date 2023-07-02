# Longdouble

This crate provides the type alias `c_longdouble` for FFI interoperability with
C's `long double`. It does not provide any new types.

The exact type that `c_longdouble` aliases to is based on 
<https://hackmd.io/@8W5l8q6-Qyyn_vKh2-L0RQ/rJpZsmcdh>. Currently the only
bindings available are for targets where `long double` is `f64`. Once the
[`f128` and `f116` RFC](https://github.com/rust-lang/rfcs/pull/3453) becomes
usable, platforms that use `f128` will be supported. If the [additional float
types RFC](https://github.com/rust-lang/rfcs/pull/3451) lands, bindings for f80
will also be supported.
