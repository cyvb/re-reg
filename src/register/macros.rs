/// This macro is used to generate a registers layout struct. This struct contains
/// registers and paddings, which matches the real hardware's layout.
/// Please refer to `reg_fields!()` macro to see how the struct is generated.
///
/// Before generating a layout, using `reg_bitfields!{}` to generate the bit fields
/// is recommended (unless you don't need any bit field at all). Please refer
/// to that macro to view details.
///
/// The fields like `FR::Reg` are "register names", which is used to match a bit field
/// control `Bits` with a specific register. This could be automatically generated when
/// you using `reg_bitfields!{}`. It guarantees that a bit field with register name
/// `FR::Reg` can be used only on that register. Please refer to `register::bitfield` mod
/// to view details.
///
/// ### Example
/// ```
/// use neo_reg::prelude::*;
/// reg_bitfields! {
///     FR(u32) [ FR1 [ 0 => 8 ] ],
///     DR(u32) [ DR1 [ 0 => 8 ] ]
/// }
/// registers_layout! {
///     /* Add attributes here. */
///     /* The struct has attrs `repr(C)` and `allow(non_snake_case)` by default */
///     MyDeviceRegs {
///         /* Every layout should start from 0x00 */
///         ( 0x00 => FR: RO<u32, FR::Reg> ),
///         ( 0x04 => DR: RW<u32, DR::Reg> ),
///         /* Padding field, from 0x08 to 0x0f is marked reserved */
///         ( 0x08 => _reserved0 ),
///         /* Omit the bitfield is okay for regs that don't need bit-field operations */
///         ( 0x10 => CR: WO<u32> ),
///         /* @END indicates the layout ends here */
///         @END
///         /* The range it represents is from 0x00 to 0x13 */
///     }
/// }
/// ```
///
/// TODO:
/// - Add compile time check for parameter validity.
///
#[macro_export]
macro_rules! registers_layout {
    {
        $(#[$attr:meta])*
        $name:ident {
            $($field:tt)*
        }
    } => {
        $crate::reg_fields!(
            $(#[$attr])*
            $name {
                $($field)*
            }
        );
    }
}

#[macro_export]
macro_rules! reg_fields {
    /* Parsing struct header. */
    (
        $(#[$attr:meta])*
        $name:ident {
            $($fieled:tt)*
        }
    ) => {
        $crate::reg_fields!(
            ( $($fieled)* ) -> {
                $(#[$attr])*
                struct $name;
            }
        );
    };
    /* Parsing paddings. */
    (
        (
            ($offset:literal => $padding:ident),
            ($offset_next:literal => $($field_next:tt)*),
            $($other:tt)*
        ) -> { $($out:tt)* }
    ) => {
        $crate::reg_fields!(
            (
                ($offset_next => $($field_next)*),
                $($other)*
            ) -> {
                $($out)*
                ($padding: [u8; $offset_next - $offset]),
            }
        );
    };
    /* Parsing read-only regs. */
    (
        (
            ($offset:literal => $name:ident: RO<$typ:ty$(, $rname:path)?>),
            $($other:tt)*
        ) -> { $($out:tt)* }
    ) => {
        $crate::reg_fields!(
            ( $($other)* ) -> {
                $($out)*
                ($name: ROInnerRegister<$typ$(, $rname)?>),
            }
        );
    };
    /* Parsing write-only regs. */
    (
        (
            ($offset:literal => $name:ident: WO<$typ:ty$(, $rname:path)?>),
            $($other:tt)*
        ) -> { $($out:tt)* }
    ) => {
        $crate::reg_fields!(
            ( $($other)* ) -> {
                $($out)*
                ($name: WOInnerRegister<$typ$(, $rname)?>),
            }
        );
    };
    /* Parsing read-write regs. */
    (
        (
            ($offset:expr => $name:ident: RW<$typ:ty$(, $rname:path)?>),
            $($other:tt)*
        ) -> { $($out:tt)* }
    ) => {
        $crate::reg_fields!(
            ( $($other)* ) -> {
                $($out)*
                ($name: RWInnerRegister<$typ$(, $rname)?>),
            }
        );
    };
    /* Finish. */
    (
        (
            @END$(,)?
        ) -> {
            $(#[$attr:meta])*
            struct $struct_name:ident;
            $(
                ($entry_name:ident: $typ:ty),
            )*
        }
    ) => {
        $(#[$attr])*
        #[repr(C)]
        #[allow(non_snake_case)]
        pub struct $struct_name {
            $(
                pub $entry_name: $typ
            ),*
        }
    };
}

/// This macro is used to generate a set of bit fields' info. It will create mods
/// in the caller file with. Each mod contains a struct named `Reg` which implemets
/// `RegName` to create a "register name". Then it implements `BisLike<$type>` to imply
/// that the bit field is used on register of type `$type`.
///
/// The rest contents in the mod are the defined bit fields and constant values.
///
/// The `assert!()` here guarantees that each field should be larger than 0 and at most the
/// same size as the register. It also guarantees the bit field doesn't exceeds the regiter's
/// boundary (e.g. a field of 6-bit but whose offset is 4 in a register of 8-bit is
/// apparently invalid, since the bits in register is [0:7] but the field is [5:10])
///
/// ### Example
/// ```
/// use neo_reg::prelude::*;
/// reg_bitfields! {
///     CR(u32) [
///         /* [ offset => field_size] */
///         DLEN [ 7 => 1 ] {
///             /* Constant values. */
///             DLen7 = 0x00,
///             DLen8 = 0x01
///         },
///         FFEN [ 0 => 1 ]
///     ],
///     DR(u32) [
///         TXD [ 0 => 16 ]
///     ]
/// }
/// ```
#[macro_export]
macro_rules! reg_bitfields {
    {
        $($rname:ident($typ:ty) [
            $($name:ident [ $offset:literal => $size:literal ] $({
                $($vname:ident = $vval:literal),*
            })?),*$(,)?
        ]),*$(,)?
    } => {
        $(
            #[allow(non_snake_case)]
            #[allow(non_upper_case_globals)]
            pub mod $rname {
                use $crate::prelude::*;
                pub struct Reg(PhantomData<$typ>);
                impl RegName for Reg {}
                impl BitsLike<$typ> for Reg {}
                $(
                    pub const $name: Bits<$typ, Reg> = {
                        assert!(
                            $size > 0 && $size <= core::mem::size_of::<$typ>() * 8,
                            "Bit field's size is not within (0, {{type_size}}], please check it"
                        );
                        assert!(
                            $size + $offset <= core::mem::size_of::<$typ>() * 8,
                            "Bit field's {{size + offset}} exceeds the register {{type_size}}"
                        );
                        Bits::new(
                            $offset,
                            (((1 as $typ).wrapping_shl($size) & ((0 as $typ).wrapping_sub(2))).wrapping_sub(1) << $offset
                        )
                    )};
                    $(
                        $(pub const $vname: $typ = $vval;)*
                    )?
                )*
            }
        )*
    };

}
