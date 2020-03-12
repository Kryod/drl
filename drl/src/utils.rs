use ndarray::{ s, Array1, Array2, Array3 };
use rand::Rng;
use ndarray_stats::QuantileExt;
use rand::distributions::Distribution;
use rand::distributions::Uniform;

#[macro_export]
macro_rules! inc_vec {
    ( $( $n:expr )? ) => {
        {
            $(
                let mut v = Vec::with_capacity($n as usize);
                for i in 0..$n {
                    v.push(i);
                }
                v
            )*
        }
    };
}

pub fn apply_for_indices<T, F>(A: &mut Array1<T>, indices: &Array1<usize>, f: F)
where F: Fn(usize, &mut T) {
    for idx in indices.iter() {
        let idx = *idx;
        f(idx, &mut A[idx]);
    }
}

pub fn apply_for_indices_2<T, F>(A: &mut Array2<T>, indices: (&Array1<usize>, &Array1<usize>), f: F)
where F: Fn((usize, usize), &mut T) {
    for a in indices.0.iter() {
        for b in indices.1.iter() {
            let idx = (*a, *b);
            f(idx, &mut A[idx]);
        }
    }
}

pub fn apply_for_indices_3<T, F>(A: &mut Array3<T>, indices: (&Array1<usize>, &Array1<usize>, &Array1<usize>), f: F)
where F: Fn((usize, usize, usize), &mut T) {
    for a in indices.0.iter() {
        for b in indices.1.iter() {
            for c in indices.2.iter() {
                let idx = (*a, *b, *c);
                f(idx, &mut A[idx]);
            }
        }
    }
}

pub fn rand_pick<T>(A: &Array1<T>) -> T
where T: Copy {
    let n = A.len();
    let mut rng = rand::thread_rng();
    A[rng.gen_range(0, n)]
}

pub fn rand_pick_greedy(A: &Array1<usize>, epsilon: f32, V: &Array2<f32>, s: usize) -> usize{
    let between = Uniform::new(0.0,1.0);
    let mut rng = rand::thread_rng();
    if between.sample(&mut rng) < epsilon {
        rand_pick(A)
    }
    else {
        V.slice(s![s, ..]).argmax().unwrap()
    }
}


pub fn contains<T>(A: &Array1<T>, s: T) -> bool
where T: PartialEq {
    for i in A.iter() {
        if *i == s {
            return true;
        }
    }
    false
}

#[test]
fn test_apply_for_indices() {
    let mut A = ndarray::arr1(&[2, 3, 4, 5]);
    let expected = ndarray::arr1(&[0, 3, 4, 0]);
    let indices = ndarray::arr1(&[0, 3]);
    apply_for_indices(&mut A, &indices, |_idx, x| *x = 0);
    assert_eq!(A, expected);
}
