use ndarray::{ s, arr1, Array, Array1, Array2, Array3, Ix2 };
use ndarray_rand::{ RandomExt, rand_distr::Uniform };
use ndarray_stats::QuantileExt;
//use rand::seq::SliceRandom;
//use rand::Rng;
use crate::{ utils, grid_world::World, inc_vec };

pub fn dyna_q<F>(
        lw: & impl World,
        step_func: F,
        gamma: Option<f32>,
        nb_iter: Option<i32>,
        n: Option<i32>,
        epsilon: Option<f32>,
        alpha: Option<f32>
    ) -> Array2<f32>
    where F: Fn(usize, usize, &Array3<f32>, &Array3<f32>, &Array1<usize>) -> (f32, usize),
    {

    let (S, A, T, P, R) = lw.get_all();

    let gamma = gamma.unwrap_or(0.99_f32);
    let nb_iter = nb_iter.unwrap_or(1_000_i32);
    let n = n.unwrap_or(50_i32);
    let alpha = alpha.unwrap_or(0.1_f32);
    let epsilon = epsilon.unwrap_or(0.1_f32);

    assert!(gamma >= 0.0_f32 && gamma < 1.0_f32);
    assert!(nb_iter > 0_i32);

    let mut Model_r = Array2::<f32>::zeros((S.shape()[0], A.shape()[0]));
    //let mut Model_r = Array::random((S.shape()[0], A.shape()[0]), Uniform::new(0.0_f32, 1.0_f32)).into_dimensionality::<Ix2>().unwrap();
    let mut Model_s = Array2::<usize>::zeros((S.shape()[0], A.shape()[0]));
    //let mut Model_s = Array::random((S.shape()[0], A.shape()[0]), Uniform::new(0.0_f32, 1.0_f32)).into_dimensionality::<Ix2>().unwrap();
    let mut V = Array::random((S.shape()[0], A.shape()[0]), Uniform::new(0.0_f32, 1.0_f32)).into_dimensionality::<Ix2>().unwrap();
    let shape = V.shape()[1];
    utils::apply_for_indices_2(&mut V, (T, &arr1(&inc_vec![shape]) ), |_idx, x| *x = 0.0);
    //let mut s0 = utils::rand_pick(&S);
    let mut prev_state = vec![];
    let mut prev_action = vec![];

    let mut s0 = S[lw.get_start_state()];

    for _ in 0..nb_iter {
        let a0 = utils::rand_pick_greedy(&A, epsilon, &V, s0);
        

        if utils::contains(&T, s0) {
            s0 = S[lw.get_start_state()];
            continue;
        }

        let (r, s_prime) = step_func(s0, a0, &P, &R, &S);
        let argmax = V.slice(s![s_prime, ..]).argmax().unwrap();
        V[(s0, a0)] = V[(s0, a0)] + alpha * (r + gamma * V[(s_prime, argmax)] - V[(s0, a0)]);
        Model_r[(s0,a0)] = r;
        Model_s[(s0,a0)] = s_prime;
        let mut t = 0;
        while t < n {
            /*if prev_state.len() == 0 {
                break;
            }
            let mut rng = rand::thread_rng();
            let n = rng.gen_range(0, prev_state.len());
            let s = prev_state[n];*/

            let s = utils::rand_pick(&S);
            let a = utils::rand_pick(&A);
            if utils::contains(T, s){
                break;
            }
            /*let s = match prev_state.choose(&mut rand::thread_rng()) {
                Some(s) => *s,
                None => break,
            };

            let a = prev_action[s];*/
            //let a = *prev_action.choose(&mut rand::thread_rng()).unwrap();
            let s_prime = Model_s[(s,a)];
            let r = Model_r[(s,a)];
            let argmax = V.slice(s![s_prime, ..]).argmax().unwrap();
            V[(s, a)] = V[(s, a)] + alpha * (r + gamma * V[(s_prime, argmax)] - V[(s, a)]);
            t+=1;

            
        }

        prev_state.push(s0);
        prev_action.push(a0);
        s0 = s_prime;
        
    }

    V
}
