use criterion::{criterion_group, criterion_main, Criterion};
use std::mem;
use std::sync::atomic::{fence, Ordering};
use aligned_array::{Aligned, A16};

pub fn offset_load<T: Copy, U: Copy, const OFFSET: isize>(array: &mut Aligned<A16, [u8; 32]>, obj: T) -> U {
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

macro_rules! register_benchmark {
    ($c:expr, $arr:expr, $store_typ:ty, $load_typ:ty, $offset:expr) => {
        if std::mem::size_of::<$load_typ>() + $offset <= std::mem::size_of::<$store_typ>() {
            $c.bench_function(
                &format!(
                    "store_{}_load_{}_offset_{}",
                    std::mem::size_of::<$store_typ>(),
                    std::mem::size_of::<$load_typ>(),
                    $offset
                ),
                |c| c.iter(|| offset_load::<$store_typ, $load_typ, $offset>($arr, 0)),
            );
        }
    };
}

macro_rules! register_benchmarks {
    ($c:expr, $arr:expr, $store_typ:ty, $load_typ:ty) => {
        register_benchmark!($c, $arr, $store_typ, $load_typ, 0);
        register_benchmark!($c, $arr, $store_typ, $load_typ, 1);
        register_benchmark!($c, $arr, $store_typ, $load_typ, 2);
        register_benchmark!($c, $arr, $store_typ, $load_typ, 3);
        register_benchmark!($c, $arr, $store_typ, $load_typ, 4);
        register_benchmark!($c, $arr, $store_typ, $load_typ, 5);
        register_benchmark!($c, $arr, $store_typ, $load_typ, 6);
        register_benchmark!($c, $arr, $store_typ, $load_typ, 7);
        register_benchmark!($c, $arr, $store_typ, $load_typ, 8);
        register_benchmark!($c, $arr, $store_typ, $load_typ, 9);
        register_benchmark!($c, $arr, $store_typ, $load_typ, 10);
        register_benchmark!($c, $arr, $store_typ, $load_typ, 11);
        register_benchmark!($c, $arr, $store_typ, $load_typ, 12);
        register_benchmark!($c, $arr, $store_typ, $load_typ, 13);
        register_benchmark!($c, $arr, $store_typ, $load_typ, 14);
        register_benchmark!($c, $arr, $store_typ, $load_typ, 15);
    };
}

fn forward_store_benchmark(c: &mut Criterion) {
    let mut arr: Aligned<A16, [u8; 32]> = Aligned([0; 32]);
    register_benchmarks!(c, &mut arr, u16, u8);
    register_benchmarks!(c, &mut arr, u16, u16);
    register_benchmarks!(c, &mut arr, u32, u8);
    register_benchmarks!(c, &mut arr, u32, u16);
    register_benchmarks!(c, &mut arr, u32, u32);
    register_benchmarks!(c, &mut arr, u64, u8);
    register_benchmarks!(c, &mut arr, u64, u16);
    register_benchmarks!(c, &mut arr, u64, u32);
    register_benchmarks!(c, &mut arr, u64, u64);
    register_benchmarks!(c, &mut arr, u128, u8);
    register_benchmarks!(c, &mut arr, u128, u16);
    register_benchmarks!(c, &mut arr, u128, u32);
    register_benchmarks!(c, &mut arr, u128, u64);
    register_benchmarks!(c, &mut arr, u128, u128);
}

criterion_group!(benches, forward_store_benchmark);
criterion_main!(benches);
