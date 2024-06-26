use super::*;

use frame_system::RawOrigin;
use frame_benchmarking::{benchmarks, whitelisted_caller, impl_benchmark_test_suite};
use sp_std::{vec, vec::Vec, boxed::Box};

#[allow(unused)]
use crate::Module as Template;

benmarks! {
    do_something {
        let s in 0 .. 100;
        let caller: T::AccountId = whitelisted_caller();
    }: _(RawOrigin::Signed(caller), s)
    verify {
        assert_eq!(Something::<T>::get(), Some(s));
    }
}

impl_benchmark_test_suite!(
    Template,
    crate::mock::new_test_ext(),
    crate::mock::Test,
);