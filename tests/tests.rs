#[macro_use]
extern crate algeoxide;

use algeoxide::*;


#[test] fn test_32_pass() { _test_pass_semigroup(123, 456, 123 + 456); }
#[test] fn test_32_fail() { _test_fail_semigroup(123, 456, 123 + 457); }
#[test] fn test_u32_pass() { _test_pass_semigroup(123u32, 456u32, 123u32 + 456u32); }

#[test] fn test_vec_i32_pass_sumoption() { _test_pass_sumoption(vec![1, 2, 3], Some(6)); }
#[test] fn test_vec_u32_pass_sumoption() { _test_pass_sumoption(vec![1u32, 2u32, 3u32], Some(6u32)); }
#[test] fn test_vec_i32_pass_sum() { _test_pass_sum(vec![1, 2, 3], 6); }
#[test] fn test_vec_u32_pass_sum() { _test_pass_sum(vec![1u32, 2u32, 3u32], 6u32); }

fn _test_pass_semigroup<T: Semigroup + Eq>(a: T, b: T, c: T) {
  let sg = a.plus(&b);
  assert!(sg == c);
}

fn _test_fail_semigroup<T: Semigroup + Eq>(a: T, b: T, c: T) {
  let sg = a.plus(&b);

  assert!(sg != c);
}


fn _test_pass_sumoption<T: Semigroup + Eq + Clone, U: SemigroupSeq<T>>(a: U, c: Option<T>) {
  let r_opt: Option<T> = Semigroup::sum_option(& a);
  assert!(r_opt == c)
}


fn _test_pass_sum<T: Monoid + Eq + Clone, U: MonoidSeq<T>>(a: U, c: T) {
  let r: T = Monoid::sum(& a);
  assert!(r == c)
}

