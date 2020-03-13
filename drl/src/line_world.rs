use ndarray::{ s, arr1, Array1, Array2, Array3 };
use rand::{ prelude::*, distributions::WeightedIndex };

use crate::grid_world::World;
use crate::inc_vec;

pub struct LineWorld {
    pub S: Array1<usize>,
    pub A: Array1<usize>,
    pub T: Array1<usize>,
    pub P: Array3<f32>,
    pub R: Array3<f32>,
}

impl World for LineWorld {
    fn get_all(&self) -> (&Array1<usize>, &Array1<usize>, &Array1<usize>, &Array3<f32>, &Array3<f32>) {
        (&self.S, &self.A, &self.T, &self.P, &self.R)
    }

    fn get_start_state(&self) -> usize {
        2
    }

    fn step(&self, s: usize, a: usize, P: &Array3<f32>, R: &Array3<f32>, S: &Array1<usize>) -> (f32, usize) {
        let dist = WeightedIndex::new(P.slice(s![s, a, ..])).unwrap();
        let mut rng = thread_rng();
        let s_p = S[dist.sample(&mut rng)];
        let r = R[(s, a, s_p)];
        (r, s_p)
    }

    fn step_until_the_end_of_episode_and_return_transitions(
            &self,
            s: usize,
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
            let (r, s_p) = self.step(s, a, P, R, S);
            s_list.push(s);
            a_list.push(a);
            r_list.push(r);
            s_p_list.push(s_p);
            s = s_p;
        }
        (s_list, a_list, r_list, s_p_list)
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
    R[(1, 0, 0)] = -5.0;
    R[(num_states - 2, 1, num_states - 1)] = 1.0;

    LineWorld { S, A, T, P, R }
}
