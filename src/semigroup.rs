
pub trait Semigroup : Clone {
  fn plus(&self, o: &Self) -> Self;

  fn sum_option<T: SemigroupSeq<Self>>(a: & T) -> Option<Self> {
    a.sum_option()
  }
}


pub trait SemigroupSeq<T> {
  fn sum_option(&self) -> Option<T>;
}

#[macro_export]
macro_rules! gen_semigroup_seq {
    ( $( $x:ty ),*  )  => {
        $(
          impl SemigroupSeq<$x> for Vec<$x> {
            fn sum_option(&self) -> Option<$x> {
              let mut iter = self.iter();
              let head_opt: Option<&$x> = iter.next();
              match head_opt {
                None => None,
                Some(h) => {
                let mut result: $x = h.clone();
                loop {
                  match iter.next() {
                    Some(x) =>
                      result = result.plus(x),
                    None => break,
                  }
                }
                Some(result)
                }
              }
            }
          }
        )*
      };
}


// Useful for simple case where rust can with type checking just use addition
macro_rules! gen_semigroup_num {
    ( $( $x:ty )* )  => {
        $(

          impl Semigroup for $x {
            fn plus(&self, o: & $x) -> $x {
              self + o
            }
          }
        )*
      };
}

gen_semigroup_num!(u8 u16 i8 i16 i32 u32);
gen_semigroup_seq!(u8, i32, i16, u32);