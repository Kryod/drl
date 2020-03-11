use ndarray::{arr1, Array1, Array3};
use crate::inc_vec;
use crate::utils;

pub trait World {
    fn new(S: Array1<usize>, A: Array1<usize>, T: Array1<usize>, P: Array3<f32>, R: Array3<f32>) -> Self;

    fn get_all(&self) -> (&Array1<usize>, &Array1<usize>, &Array1<usize>, &Array3<f32>, &Array3<f32>);
}


pub struct GridWorld {
    pub S: Array1<usize>,
    pub A: Array1<usize>,
    pub T: Array1<usize>,
    pub P: Array3<f32>,
    pub R: Array3<f32>,
}

impl World for GridWorld {
    fn new(S: Array1<usize>, A: Array1<usize>, T: Array1<usize>, P: Array3<f32>, R: Array3<f32>) -> Self {
        Self {
            S,
            A,
            T,
            P,
            R
        }
    }

    fn get_all(&self) -> (&Array1<usize>, &Array1<usize>, &Array1<usize>, &Array3<f32>, &Array3<f32>) {
        (&self.S, &self.A, &self.T, &self.P, &self.R)
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

    GridWorld::new(S, A, T, P, R)
}