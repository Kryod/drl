use ndarray::{ arr1, Array, Array2, Ix2 };
use ndarray_rand::{ RandomExt, rand_distr::Uniform };
use crate::{ utils, grid_world::World, inc_vec };

pub fn sarsa(
        w: &impl World,
        gamma: Option<f32>,
        nb_iter: Option<i32>,
        max_step: Option<i32>,
        epsilon: Option<f32>,
        alpha: Option<f32>
    ) -> Array2<f32>
    {

    let (S, A, T, P, R) = w.get_all();

    let gamma = gamma.unwrap_or(0.99_f32);
    let nb_iter = nb_iter.unwrap_or(1_000_i32);
    let max_step = max_step.unwrap_or(100_i32);
    let alpha = alpha.unwrap_or(0.1_f32);
    let epsilon = epsilon.unwrap_or(0.1_f32);

    assert!(gamma >= 0.0_f32 && gamma < 1.0_f32);
    assert!(nb_iter > 0_i32);

    let mut V = Array::random((S.shape()[0], A.shape()[0]), Uniform::new(0.0_f32, 1.0_f32)).into_dimensionality::<Ix2>().unwrap();
    let shape = V.shape()[1];
    utils::apply_for_indices_2(&mut V, (T, &arr1(&inc_vec![shape]) ), |_idx, x| *x = 0.0);

    for _ in 0..nb_iter {
        //let mut s0 = utils::rand_pick(&S);
        let mut s0 = S[w.get_start_state()];
        let mut a0 = utils::rand_pick_greedy(&A, epsilon, &V, s0);
        let mut t = 0;
        if utils::contains(&T, s0) {
            continue;
        }
        while t < max_step {
            let (r, s_prime) = w.step(s0, a0, &P, &R, &S);
            let a_prime = utils::rand_pick_greedy(&A, epsilon, &V, s_prime);
            V[(s0, a0)] = V[(s0, a0)] + alpha * (r + gamma * V[(s_prime, a_prime)] - V[(s0, a0)]);
            
            s0 = s_prime;
            a0 = a_prime;
            t+=1;
            if utils::contains(T, s0){
                break;
            }
        }
        
    }

    V
}
