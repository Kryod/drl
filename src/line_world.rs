use ndarray::{ s, arr1, Array1, Array2, Array3 };
use rand::{ prelude::*, distributions::WeightedIndex };

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
/*
def step(s: int, a: int) -> (float, int):
    s_p = np.random.choice(S, p=P[s, a, :])
    r = R[s, a, s_p]
    return r, s_p


def step_until_the_end_of_episode_and_return_transitions(s: int, Pi: np.ndarray) -> \
        ([int], [int], [float], [int]):
    s_list = []
    a_list = []
    r_list = []
    s_p_list = []
    while s not in T and len(s_list) < len(S) * 10:
        a = np.random.choice(A, p=Pi[s])
        r, s_p = step(s, a)
        s_list.append(s)
        a_list.append(a)
        r_list.append(r)
        s_p_list.append(s_p)
        s = s_p
    return s_list, a_list, r_list, s_p_list*/

pub struct LineWorld {
    pub S: Array1<usize>,
    pub A: Array1<usize>,
    pub T: Array1<usize>,
    pub P: Array3<f32>,
    pub R: Array3<f32>,
}

impl LineWorld {
    fn new(S: Array1<usize>, A: Array1<usize>, T: Array1<usize>, P: Array3<f32>, R: Array3<f32>) -> Self {
        Self {
            S,
            A,
            T,
            P,
            R
        }
    }
}

pub fn init(num_states: usize) -> LineWorld {
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

    LineWorld::new(S, A, T, P, R)
}


pub fn step(s: usize, a: usize, P: &Array3<f32>, R: &Array3<f32>, S: &Array1<usize>) -> (f32, usize){
    let dist = WeightedIndex::new(P.slice(s![s, a, ..])).unwrap();
    let mut rng = thread_rng();
    let s_p = S[dist.sample(&mut rng)];
    let r = R[[s, a, s_p]];
    (r, s_p)
}

pub fn step_until_the_end_of_episode_and_return_transitions(s: usize,
        Pi: &Array2<f32>,
        S: &Array1<usize>,
        A: &Array1<usize>,
        T: &Array1<usize>,
        P: &Array3<f32>,
        R: &Array3<f32> 
    ) ->
        (Vec<usize>, Vec<usize>, Vec<f32>, Vec<usize>) {
    let mut s_list = vec![];
    let mut a_list = vec![];
    let mut r_list = vec![];
    let mut s_p_list = vec![];
    let mut rng = thread_rng();
    let mut s = s;
    while !crate::utils::contains(&T, s) && s_list.len() < S.len() * 10 {
        let dist = WeightedIndex::new(&Pi.slice(s![s, ..])).unwrap();
        let a = A[dist.sample(&mut rng)];
        let (r, s_p) = step(s, a, P, R, S);
        s_list.push(s);
        a_list.push(a);
        r_list.push(r);
        s_p_list.push(s_p);
        s = s_p;
    }
    (s_list, a_list, r_list, s_p_list)
}
