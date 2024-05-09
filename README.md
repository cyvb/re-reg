# re-reg (stylized as _re.g_)

Yet another crate for register operations. Inspired by [tock-registers](https://github.com/tock/tock/tree/master/libraries/tock-register-interface).

By default `std` support is on, turning off this feature by adding `default-features = false` to this crate's config in your `Cargo.toml`.

Usage example:

Create bit fields.

```rust
reg_bitfields! {
    // This is the {reg_name}.
    Reg1(u32) [
        Field1 [ 0 => 2 ], // [ {offset} => {size} ]
        Field2 [ 2 => 4 ],
        Flag1  [ 7 => 1 ]
    ]
}
```

Create registers layout.

```rust
registers_layout! {
    RegLayout {
        ( 0x00 => Reg1: RW<u32, FT::Reg> ), // Always {reg_name}::Reg.
        @END                                // Don't miss the "@END".
    }
}
```

And operate!

```rust
// Put 0b01 into Field1, put 0b10 into Field2 and set Flag1.
// Keep the other bits untouched.
RegLayout.Reg1.write_back(
    Reg1::Field1.val(0b01)
    + Reg1::Field2.val(0b10)
    + Reg1::Flag
);
```
