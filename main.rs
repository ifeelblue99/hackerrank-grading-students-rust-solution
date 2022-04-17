enum GradeResult {
  Fail,
  Round(i8),
  Leave,
}
fn main() {
  let grades: [i8;3] = [84, 29, 57];
  let mut range = 0..grades.len();
  loop {
    match range.next() {
      Some(x) => {//
        match round(grades[x]) {
          GradeResult::Fail => println!("Grade: {} is Failed!", grades[x]),
          GradeResult::Leave => println!("Grade: {} is left as it is.", grades[x]),
          GradeResult::Round(num) => println!("Grade: {} is rounded to: {}.", grades[x], num),
        }
        //
      },
      None => break,
    }
  }
}
fn round(num: i8) -> GradeResult {
  use GradeResult::*;
  if num < 38  {
    return Fail
  }
  if (-num%5 + 5) >= 3{
    return Leave  
  } else {
    return Round(-num%5 + num +5 )
  }
}
