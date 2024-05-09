
use std::time::Instant;
use std::io::{stdin, stdout, Write};

use re_reg::prelude::*;

#[allow(dead_code)]
struct B {
    v: u32
}

reg_bitfields! {
    FT(u32) [
        B1 [ 0 => 2 ],
        B2 [ 2 => 4 ]
    ]
}

registers_layout! {
    RT {
        ( 0x00 => V: RW<u32, FT::Reg> ),
        @END
    }
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

#[allow(dead_code)]
fn get_input() -> u32 {
    let mut s = String::new();
    print!("Please enter some text: ");
    let _ = stdout().flush();
    stdin().read_line(&mut s).expect("Did not enter a correct string");
    if let Some('\n') = s.chars().next_back() {
        s.pop();
    }
    if let Some('\r') = s.chars().next_back() {
        s.pop();
    }

    s.parse::<u32>().unwrap()
}

trait RegLike {
    fn p(self, input: u32, off: usize, mask: u32) -> Self;
    fn cl(self, mask1: u32, mask2: u32) -> Self;
}

impl RegLike for u32 {
    //#[inline]
    fn p(mut self, input: u32, off: usize, mask: u32) -> Self {
        self |= (input << off) & mask;
        self
    }
    //#[inline]
    fn cl(mut self, mask1: u32, mask2: u32) -> Self {
        self &= !(mask1 | mask2);
        self
    }
}

#[test]
fn perftest() {
    let a = B { v: 0 };
    let io = IO::<RT>::new(&a as *const _ as usize);
    let mut b = 0u32;

    // let num = get_input();
    // println!("Get {}", num);

    let range = 0..100000;
    let t1 = Instant::now();
    {
        for n in range {
            io.V.put_back(FT::B1.val(n) + FT::B2.val(n));
            io.V.clear(FT::B1 + FT::B2);
        }
    }
    let t2 = Instant::now();
    println!("time = {:?}", t2 - t1);

    let start2 = Instant::now();
    {
        for m in 0..100000 {
            b = b.p(m, 0, 0b11)
                .p(m, 2, 0b1100);
        }
    }
    let end2 = Instant::now();
    println!("time = {:?}", end2 - start2);
}
