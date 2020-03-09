use ndarray::{arr1, Array1, Array3};
use crate::inc_vec;

//num_states = 100
//S = np.arange(num_states)
//A = np.array([0, 1])  # 0: left, 1 : right
//T = np.array([0, num_states - 1])
//P = np.zeros((len(S), len(A), len(S)))
//R = np.zeros((len(S), len(A), len(S)))

//for s in S[1:-1]:
//    P[s, 0, s - 1] = 1.0
//    P[s, 1, s + 1] = 1.0

//R[1, 0, 0] = -1.0
//R[num_states - 2, 1, num_states - 1] = 1.0

pub fn init(num_states: usize) -> (Array1<usize>, Array1<usize>, Array1<usize>, Array3<f32>, Array3<f32>) {
    let S = arr1(&inc_vec![num_states]);
    let A = arr1(&[0,1]);
    let T = arr1(&[0,num_states - 1]);
    let mut P = Array3::<f32>::zeros((S.shape()[0], A.shape()[0], S.shape()[0]));
    let mut R = Array3::<f32>::zeros((S.shape()[0], A.shape()[0], S.shape()[0]));

    for s in S.iter() {
        if *s == 0 {
            continue;
        }
        if *s == S.shape()[0]-1 {
            continue;
        }
        P[(*s, 0, *s-1)] = 1.0;
        P[(*s, 1, *s+1)] = 1.0;
    }
    R[(1, 0, 0)] = -1.0;
    R[(num_states - 2, 1, num_states - 1)] = 1.0;

    (S, A, T, P, R)
}
