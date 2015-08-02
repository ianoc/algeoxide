#![feature(test)]
#![feature(core)]
#[macro_use]


extern crate algeoxide;
extern crate test;
extern crate core;
use algeoxide::*;
use test::Bencher;
use core::fmt::Debug;
#[bench] fn sum_1024_i8(bencher: &mut Bencher) { _bench_sum_option::<i8, Vec<i8>>(bencher, (0..1024).collect()); }
#[bench] fn sum_1024_i16(bencher: &mut Bencher) { _bench_sum_option::<i16, Vec<i16>>(bencher, (0..1024).collect()); }
#[bench] fn sum_1024_i32(bencher: &mut Bencher) { _bench_sum_option::<i32, Vec<i32>>(bencher, (0..1024).collect()); }


#[bench] fn sum_1024_u8(bencher: &mut Bencher) { _bench_sum_option::<u8, Vec<u8>>(bencher, (0..1024).collect()); }
#[bench] fn sum_1024_u16(bencher: &mut Bencher) { _bench_sum_option::<u16, Vec<u16>>(bencher, (0..1024).collect()); }
#[bench] fn sum_1024_u32(bencher: &mut Bencher) { _bench_sum_option::<i32, Vec<i32>>(bencher, (0..1024).collect()); }

#[derive(Clone, Debug, Eq, PartialEq)]
struct MyBox {
  a: i32
}



impl Semigroup for MyBox {
  fn plus(&self, o: &MyBox) -> MyBox {
    MyBox {
      a: self.a + o.a
    }
  }
}

gen_semigroup_seq!(MyBox);

#[bench] fn sum_1024_my_box(bencher: &mut Bencher) {
  let input_data: Vec<MyBox> = (0..1024).map(|x| MyBox { a: x } ).collect();
  _bench_sum_option::<MyBox, Vec<MyBox>>(bencher, input_data);
}


fn _bench_sum_option<T: Semigroup + Eq + Clone + Debug, U: SemigroupSeq<T>>(bencher: &mut Bencher, record: U) {
    bencher.iter(|| {
        let res: Option<T> = Semigroup::sum_option(& record);
        res
    });
}

