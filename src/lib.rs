//tests file
mod utils;

use utils::{algebra,geometry,miscellaneous,trig};
use algebra::*;

#[cfg(test)]

mod tests{
  use super::*;
  //testing algebra functions:
  
  #[test]
  fn test_factorial(){
    assert_eq!(factorial(5),120);
    assert_eq!(factorial(0),1);
  }

  #[test]
  fn test_power(){
    assert_eq!(power(2.0,3.0),8.0);
    assert_eq!(power(5.0,0.0),1.0);
  }
}