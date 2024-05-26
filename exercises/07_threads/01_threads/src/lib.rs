// TODO: implement a multi-threaded version of the `sum` function
//  using `spawn` and `join`.
//  Given a vector of integers, split the vector into two halves and
//  sum each half in a separate thread.

// Caveat: We can't test *how* the function is implemented,
// we can only verify that it produces the correct result.
// You _could_ pass this test by just returning `v.iter().sum()`,
// but that would defeat the purpose of the exercise.
//
// Hint: you won't be able to get the spawned threads to _borrow_
// slices of the vector directly. You'll need to allocate new
// vectors for each half of the original vector. We'll see why
// this is necessary in the next exercise.
use std::thread;

pub fn sum(v: Vec<i32>) -> i32 {
    let sp_size = v.len()/2;
    let (vec_1, vec_2) = v.split_at(sp_size);
    let vec_1 = vec_1.to_vec();
    let vec_2 = vec_2.to_vec();
    let sum_1 = spawn(move|| {
        vec_1.into_iter().sum::<i32>()
    });
    let sum_2 = spawn(move|| {
        vec_2.into_iter().sum::<i32>()
    });
    sum_1.join().unwrap() + sum_2.join().unwrap()
    //println!("{:?} and {:?}", slice_1, slice_2);
    //0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run() {
        sum(vec![1,2,3,4,5]);
    }

    #[test]
    fn empty() {
        assert_eq!(sum(vec![]), 0);
    }

    #[test]
    fn one() {
        assert_eq!(sum(vec![1]), 1);
    }

    #[test]
    fn five() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5]), 15);
    }

    #[test]
    fn nine() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]), 45);
    }

    #[test]
    fn ten() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 55);
    }
}
