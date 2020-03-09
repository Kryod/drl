use ndarray::{ Array1, Array2, Array3 };

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

#[test]
fn test_apply_for_indices() {
    let mut A = ndarray::arr1(&[2, 3, 4, 5]);
    let expected = ndarray::arr1(&[0, 3, 4, 0]);
    let indices = ndarray::arr1(&[0, 3]);
    apply_for_indices(&mut A, &indices, |_idx, x| *x = 0);
    assert_eq!(A, expected);
}
