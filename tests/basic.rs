
use re_reg::prelude::*;

#[test]
fn test_read() {
    let a = A { v1: 0, v2: 0b11100000 };
    let io = IO::<RA>::new(&a as *const _ as usize);

    assert_eq!(io.VY.read(), 0b11100000);
}

#[test]
fn test_get_bits() {
    let a = A { v1: 0b11000011, v2: 0b10100000 };
    let io = IO::<RA>::new(&a as *const _ as usize);

    assert_eq!(io.VY.get(F2::B1), 1);
    assert_eq!(io.VY.get(F2::B2), 0);
    assert_eq!(io.VY.get(F2::B3), 1);
    assert_eq!(io.VY.get(F2::B1 + F2::B2 + F2::B3), 0b101);
    assert_eq!(io.VX.get(F1::B1), 0b11);
}

#[test]
fn test_write() {
    let a = A { v1: 0, v2: 0 };
    let io = IO::<RA>::new(&a as *const _ as usize);

    io.VX.write(0b11010010);
    assert_eq!(io.VX.read(), 0b11010010);
}

#[test]
fn test_write_combination() {
    let a = A { v1: 0, v2: 0 };
    let io = IO::<RA>::new(&a as *const _ as usize);

    io.VX.put_back(
        F1::B1.val(F1::B1State1)
        + F1::B2.val(0b11)
        + F1::B3.val(0b11)
    );
    assert_eq!(io.VX.read(), 0b01101111);
}

#[test]
fn test_set_bits() {
    let a = A { v1: 0, v2: 0 };
    let io = IO::<RA>::new(&a as *const _ as usize);

    io.VX.put_back(
        F1::B2
        + F1::B1.val(F1::B1State2)
        + F1::B3
    );
    assert_eq!(io.VX.read(), 0b01101101);
    io.VX.set_all();
    assert_eq!(io.VX.read(), 0xffffffff);
}

#[test]
fn test_clear() {
    let a = A { v1: 0xffffffff, v2: 0 };
    let io = IO::<RA>::new(&a as *const _ as usize);

    io.VX.clear(F1::B2 + F1::B3 + F1::B1);
        // .clear(F1::B1);
    assert_eq!(io.VX.read(), 0xffffff90);
    io.VX.clear_all();
    assert_eq!(io.VX.read(), 0);
}

#[allow(dead_code)]
struct A {
    pub v1: u32,
    pub v2: u16
}

struct IO<T> {
    base: usize,
    _daio: PhantomData<*const T>
}

impl<T> IO<T> {
    pub const fn new(base: usize) -> Self {
        Self {
            base,
            _daio: PhantomData
        }
    }
}

impl<T> core::ops::Deref for IO<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe {
            &*(self.base as *const _)
        }
    }
}

reg_bitfields! {
    F1(u32) [
        B1 [ 0 => 2 ] {
            B1State1 = 0b11,
            B1State2 = 0b01
        },
        B2 [ 2 => 2 ],
        B3 [ 5 => 2 ]
    ],
    F2(u16) [
        B1 [ 5 => 1 ],
        B2 [ 6 => 1 ],
        B3 [ 7 => 1 ],
    ]
}

registers_layout! {
    RA {
        ( 0x00 => VX: RW<u32, F1::Reg> ),
        ( 0x04 => VY: RO<u16, F2::Reg> ),
        @END
    }
}
