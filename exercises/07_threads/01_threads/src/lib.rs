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
    let vec_len = v.len();
    let left = v[..vec_len / 2].to_vec();
    let right = v[vec_len / 2..].to_vec();
    let handleLeft = thread::spawn(move || {
        let total = left.iter().sum();
        Result::<i32, ()>::Ok(total)
    });

    let handleRight = thread::spawn(move || {
        let total = right.iter().sum();
        Result::<i32, ()>::Ok(total)
    });
    let Ok(sum_left) = handleLeft.join().unwrap() else {
        panic!("left thread panicked");
    };

    let Ok(sum_right) = handleRight.join().unwrap() else {
        panic!("right thread panicked");
    };

    sum_left + sum_right
}

#[cfg(test)]
mod tests {
    use super::*;

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
