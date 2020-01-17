extern crate num_bigint as bigint;
extern crate num_traits;
#[macro_use] extern crate tramp;

use bigint::BigUint;
use num_traits::{Zero, One};
use tramp::{tramp, Rec};

fn fib(n: i32) -> BigUint {
	tramp(do_fib(n, Zero::zero(), One::one()))
}

fn do_fib(n: i32, acc: BigUint, curr: BigUint) -> Rec<BigUint> {
	if n <= 0 {
		rec_ret!(acc)
	}else{
		let new = &acc + curr;
		let nn = n - 1;
		rec_call!(do_fib(nn, new, acc))
	}
}

fn main() {
	let n = 1000000;
	println!("{}", fib(n));
}