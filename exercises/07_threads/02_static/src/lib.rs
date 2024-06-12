// TODO: Given a static slice of integers, split the slice into two halves and
//  sum each half in a separate thread.
//  Do not allocate any additional memory!
use std::thread;

pub fn sum(slice: &'static [i32]) -> i32 {
    let size = slice.len() / 2;
    
    //let slice_1 = &slice[0..size];
    //let slice_2 = &slice[size..];
    let (slice_1, slice_2) = slice.split_at(size);
    
    let sum_1 = thread::spawn(move|| {
        slice_1.iter().sum::<i32>()
    });
    let sum_2 = thread::spawn(move|| {
        slice_2.iter().sum::<i32>()
    });
    sum_1.join().unwrap() + sum_2.join().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run() {
        sum(&[1,2,3,4,5]);
    }
    #[test]
    fn empty() {
        static ARRAY: [i32; 0] = [];
        assert_eq!(sum(&ARRAY), 0);
    }

    #[test]
    fn one() {
        static ARRAY: [i32; 1] = [1];
        assert_eq!(sum(&ARRAY), 1);
    }

    #[test]
    fn five() {
        static ARRAY: [i32; 5] = [1, 2, 3, 4, 5];
        assert_eq!(sum(&ARRAY), 15);
    }

    #[test]
    fn nine() {
        static ARRAY: [i32; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 9];
        assert_eq!(sum(&ARRAY), 45);
    }

    #[test]
    fn ten() {
        static ARRAY: [i32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        assert_eq!(sum(&ARRAY), 55);
    }
}
