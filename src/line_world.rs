use ndarray::{arr1, Array1, Array3};

//S = np.array([0, 1, 2, 3, 4])
//A = np.array([0, 1])  # 0: left, 1 : right
//T = np.array([0, 4])
//P = np.zeros((len(S), len(A), len(S)))
//R = np.zeros((len(S), len(A), len(S)))

//for s in S[1:-1]:
//    P[s, 0, s - 1] = 1.0
//    P[s, 1, s + 1] = 1.0

//R[1, 0, 0] = -1.0
//R[3, 1, 4] = 1.0
pub fn init() -> (Array1<usize>, Array1<i32>, Array1<i32>, Array3<f32>, Array3<f32>){
    let S = arr1(&[0,1,2,3,4]);
    let A = arr1(&[0,1]);
    let T = arr1(&[0,4]);
    let mut P = Array3::<f32>::zeros((S.shape()[0], A.shape()[0], S.shape()[0]));
    let mut R = Array3::<f32>::zeros((S.shape()[0], A.shape()[0], S.shape()[0]));

    for s in S.iter() {
        if *s == 0 {
            continue;
        }
        if *s == S.shape()[0]-1 {
            continue;
        }
        P[(*s, 0, *s-1)] = -1.0;
        P[(*s, 0, *s+1)] = 1.0;
    }
    R[(1, 0, 0)] = -1.0;
    R[(3, 1, 4)] = 1.0;

    (S, A, T, P, R)
}