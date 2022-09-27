use aligned_array::{Aligned, A16};
use criterion::{criterion_group, criterion_main, Criterion};
use std::{
    mem,
    sync::atomic::{fence, Ordering},
};

pub fn offset_load<T: Copy, U: Copy, const OFFSET: isize>(
    array: &mut Aligned<A16, [u8; 32]>,
    obj: T,
) -> U {
    let mut v: U = unsafe {
        std::ptr::read_unaligned::<U>((array.as_ptr() as *const u8).offset(OFFSET) as *const U)
    };
    // Rust vectorizes code in lambda, so let's relax it.
    for _i in 0..1000 {
        unsafe {
            std::ptr::write_unaligned::<T>(array.as_mut_ptr() as *mut T, obj);
        };
        fence(Ordering::Acquire);
        v = unsafe {
            std::ptr::read_volatile::<U>((array.as_ptr() as *const u8).offset(OFFSET) as *const U)
        };
    }
    v
}

fn register_benchmark<Store: Copy + Default, Load: Copy, const OFFSET: isize>(
    c: &mut Criterion,
    arr: &mut Aligned<A16, [u8; 32]>,
) {
    if std::mem::size_of::<Load>() + OFFSET as usize <= std::mem::size_of::<Store>() {
        c.bench_function(
            &format!(
                "store_{}_load_{}_offset_{}",
                std::mem::size_of::<Store>(),
                std::mem::size_of::<Load>(),
                OFFSET
            ),
            |c| c.iter(|| offset_load::<Store, Load, OFFSET>(arr, Store::default())),
        );
    }
}

fn register_benchmarks<Store: Copy + Default, Load: Copy>(
    c: &mut Criterion,
    arr: &mut Aligned<A16, [u8; 32]>,
) {
    if std::mem::size_of::<Store>() < std::mem::size_of::<Load>() {
        return;
    }
    register_benchmark::<Store, Load, 0>(c, arr);
    register_benchmark::<Store, Load, 1>(c, arr);
    register_benchmark::<Store, Load, 2>(c, arr);
    register_benchmark::<Store, Load, 3>(c, arr);
    register_benchmark::<Store, Load, 4>(c, arr);
    register_benchmark::<Store, Load, 5>(c, arr);
    register_benchmark::<Store, Load, 6>(c, arr);
    register_benchmark::<Store, Load, 7>(c, arr);
    register_benchmark::<Store, Load, 8>(c, arr);
    register_benchmark::<Store, Load, 9>(c, arr);
    register_benchmark::<Store, Load, 10>(c, arr);
    register_benchmark::<Store, Load, 11>(c, arr);
    register_benchmark::<Store, Load, 12>(c, arr);
    register_benchmark::<Store, Load, 13>(c, arr);
    register_benchmark::<Store, Load, 14>(c, arr);
    register_benchmark::<Store, Load, 15>(c, arr);
}

macro_rules! register_all {
    ( $c: expr, $arr: expr, $( $x: ty ),+ ) => {
        register_all!(@inner $c, $arr, $($x),+ => $($x),+);
    };
    (@inner $c: expr, $arr: expr, $x: ty, $($xx: ty),+ => $($yy: ty),+) => {
        register_all!(@inner $c, $arr, $x => $($yy),+);
        register_all!(@inner $c, $arr, $($xx),+ => $($yy),+);
    };
    (@inner $c: expr, $arr: expr, $x: ty => $y: ty, $($yy: ty),+) => {
        register_all!(@inner $c, $arr, $x => $y);
        register_all!(@inner $c, $arr, $x => $($yy),+);
    };
    (@inner $c: expr, $arr: expr, $x: ty => $y: ty) => {
        register_benchmarks::<$x, $y>($c, $arr);
    };
}

fn forward_store_benchmark(c: &mut Criterion) {
    let mut arr: Aligned<A16, [u8; 32]> = Aligned([0; 32]);
    register_all!(c, &mut arr, u8, u16, u32, u64, u128);
}

criterion_group!(benches, forward_store_benchmark);
criterion_main!(benches);
