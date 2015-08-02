use semigroup::Semigroup;

pub trait Monoid : Semigroup {
  fn zero() -> Self;

  fn sum<T: MonoidSeq<Self>>(a: & T) -> Self {
    a.sum()
  }

}


pub trait MonoidSeq<T> {
  fn sum(&self) -> T;
}

#[macro_export]
macro_rules! gen_monoid_seq {
    ( $( $x:ty => $y:expr ),*  )  => {
        $(
            impl MonoidSeq<$x> for Vec<$x> {
            fn sum(&self) -> $x {
              let mut iter = self.iter();

              let mut result: $x = match iter.next() {
                None => $y(),
                Some(s) => s.clone()
              };

              loop {
                match iter.next() {
                  Some(x) =>
                    result = result.plus(x),
                  None => break,
                }
              }
              result
            }
            }
        )*
      };
}

// Useful for simple case where rust can typecheck 0 into thecorrect Monoid type
macro_rules! gen_monoid_num {
    ( $( $x:ty ),* )  => {
        $(
          impl Monoid for $x {
            fn zero() -> $x {
              0
            }
          }
        )*
      };
}



gen_monoid_num!(u8, u16, i8, i16, i32, u32);
gen_monoid_seq!(
  u8 => u8::zero,
  u16 => u16::zero,
  u32 => u32::zero,
  i8 => i8::zero,
  i16 => i16::zero,
  i32 => i32::zero);
