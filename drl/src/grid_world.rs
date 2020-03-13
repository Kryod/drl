use ndarray::{ s, arr1, Array1, Array2, Array3 };
use rand::{ prelude::*, distributions::WeightedIndex };
use crate::inc_vec;
use crate::utils;

pub trait World {
    fn get_all(&self) -> (&Array1<usize>, &Array1<usize>, &Array1<usize>, &Array3<f32>, &Array3<f32>);
    fn get_start_state(&self) -> usize;
    fn step(&self, s: usize, a: usize, P: &Array3<f32>, R: &Array3<f32>, S: &Array1<usize>) -> (f32, usize);
    fn step_until_the_end_of_episode_and_return_transitions(&self, s: usize, Pi: &Array2<f32>, S: &Array1<usize>, A: &Array1<usize>, T: &Array1<usize>, P: &Array3<f32>, R: &Array3<f32>) -> (Vec<usize>, Vec<usize>, Vec<f32>, Vec<usize>);
}

impl<W: ?Sized> World for Box<W> where W: World {
    fn get_all(&self) -> (&Array1<usize>, &Array1<usize>, &Array1<usize>, &Array3<f32>, &Array3<f32>) {
        (**self).get_all()
    }

    fn get_start_state(&self) -> usize {
        (**self).get_start_state()
    }

    fn step(&self, s: usize, a: usize, P: &Array3<f32>, R: &Array3<f32>, S: &Array1<usize>) -> (f32, usize) {
        (**self).step(s, a, P, R, S)
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
    ) -> (Vec<usize>, Vec<usize>, Vec<f32>, Vec<usize>) {
        (**self).step_until_the_end_of_episode_and_return_transitions(s, Pi, S, A, T, P, R)
    }
}

pub struct GridWorld {
    pub S: Array1<usize>,
    pub A: Array1<usize>,
    pub T: Array1<usize>,
    pub P: Array3<f32>,
    pub R: Array3<f32>,
}

impl World for GridWorld {
    fn get_all(&self) -> (&Array1<usize>, &Array1<usize>, &Array1<usize>, &Array3<f32>, &Array3<f32>) {
        (&self.S, &self.A, &self.T, &self.P, &self.R)
    }

    fn get_start_state(&self) -> usize {
        0
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

pub fn init(width: usize, height: usize) -> GridWorld {

    let num_states = width*height;
    let S = arr1(&inc_vec![num_states]);
    let A = arr1(&[0, 1, 2, 3]); // 0: left, 1 : right, 2: up, 3: down
    let T = arr1(&[width - 1, num_states-1]);
    let mut P = Array3::<f32>::zeros((S.shape()[0], A.shape()[0], S.shape()[0]));
    let mut R = Array3::<f32>::zeros((S.shape()[0], A.shape()[0], S.shape()[0]));

    for s in S.iter() {
        if *s % width == 0 {
            P[(*s, 0, *s)] = 1.0;
        }
        else {
            P[(*s, 0, *s - 1)] = 1.0;
        }
        if (*s + 1) % width == 0 {
            P[(*s, 1, *s)] = 1.0;
        }
        else {
            P[(*s, 1, *s + 1)] = 1.0;
        }
        if *s < width {
            P[(*s, 2, *s)] = 1.0;
        }
        else {
            P[(*s, 2, *s - width)] = 1.0;
        }
        if *s >= (num_states - width) {
            P[(*s, 3, *s)] = 1.0;
        }
        else {
            P[(*s, 3, *s + width)] = 1.0;
        }
    }


    let P_shape_1 = P.shape()[1];
    let P_shape_2 = P.shape()[2];
    
    utils::apply_for_indices_3(&mut P, 
        (&ndarray::arr1(&[width-1]), &ndarray::arr1(&inc_vec![P_shape_1]), &ndarray::arr1(&inc_vec![P_shape_2])), 
        |(_a, _b, _c), x| *x = 0.0);
    utils::apply_for_indices_3(&mut P, 
        (&ndarray::arr1(&[num_states-1]), &ndarray::arr1(&inc_vec![P_shape_1]), &ndarray::arr1(&inc_vec![P_shape_2])), 
        |(_a, _b, _c), x| *x = 0.0);

    let R_shape_0 = R.shape()[0];
    let R_shape_1 = R.shape()[1];

    utils::apply_for_indices_3(&mut R, 
        (&ndarray::arr1(&inc_vec![R_shape_0]), &ndarray::arr1(&inc_vec![R_shape_1]), &ndarray::arr1(&[width-1])), 
        |(_a, _b, _c), x| *x = -5.0);
    utils::apply_for_indices_3(&mut R, 
        (&ndarray::arr1(&inc_vec![R_shape_0]), &ndarray::arr1(&inc_vec![R_shape_1]), &ndarray::arr1(&[num_states-1])), 
        |(_a, _b, _c), x| *x = 1.0);

    GridWorld { S, A, T, P, R }
}
