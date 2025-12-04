// TODO: Given a vector of integers, leak its heap allocation.
//  Then split the resulting static slice into two halves and
//  sum each half in a separate thread.
//  Hint: check out `Vec::leak`.

use std::thread;

pub fn sum(v: Vec<i32>) -> i32 {
    let vec_len = v.len();
    let newv = v.leak();
    let left = &newv[..vec_len / 2];
    let right = &newv[vec_len / 2..];
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
