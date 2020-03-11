use ndarray::{ s, arr1, Array, Array1, Array2, Array3, Ix2 };
use ndarray_rand::{ RandomExt, rand_distr::Uniform };
use ndarray_stats::QuantileExt;
use crate::{ utils, policies, grid_world::World, inc_vec };

pub fn monte_carlo_control_off_policy<F, Fi>(
        lw: & impl World,
        step_func: F,
        step_until_the_end_and_return_transitions_func: Fi,
        gamma: Option<f32>,
        nb_iter: Option<i32>
    ) -> (Array2<f32>, Array1<f32>)
    where F: Fn(usize, usize, &Array3<f32>, &Array3<f32>, &Array1<usize>) -> (f32, usize), 
        Fi: Fn(usize,
            &Array2<f32>,
            &Array1<usize>,
            &Array1<usize>,
            &Array1<usize>,
            &Array3<f32>,
            &Array3<f32> ) -> (Vec<usize>, Vec<usize>, Vec<f32>, Vec<usize>)
     
    {

    let (S, A, T, P, R) = lw.get_all();

    let gamma = gamma.unwrap_or(0.99_f32);
    let nb_iter = nb_iter.unwrap_or(1_000_i32);

    assert!(gamma >= 0.0_f32 && gamma < 1.0_f32);
    assert!(nb_iter > 0_i32);

    let mut Pi = Array1::<f32>::zeros(S.shape()[0]);
    let mut V = Array::random((S.shape()[0], A.shape()[0]), Uniform::new(0.0_f32, 1.0_f32)).into_dimensionality::<Ix2>().unwrap();
    let mut C = Array2::<f32>::zeros((S.shape()[0], A.shape()[0]));
    let shape = V.shape()[1];
    utils::apply_for_indices_2(&mut V, (T, &arr1(&inc_vec![shape]) ), |_idx, x| *x = 0.0);

    for i in 0 .. V.shape()[0] {
        let argmax = V.slice(s![i, ..]).argmax().unwrap();
        Pi[i] = V[(i, argmax)];
    }

    for _ in 0..nb_iter {
        let s0 = utils::rand_pick(&S);

        let B = policies::create_random_uniform_policy(S.len(), A.len());
        if utils::contains(&T, s0) {
            continue;
        }
        let a0 = utils::rand_pick(&A);
        let (r, s) = step_func(s0, a0, &P, &R, &S);
        let (mut s_list, mut a_list, mut r_list, _) = step_until_the_end_and_return_transitions_func(s, &B, &S, &A, &T,&P,&R);
        let mut G = 0.0;
        let mut W = 1.0;
        s_list.insert(0, s0);
        a_list.insert(0, a0);
        r_list.insert(0, r);
        for t in (0..s_list.len()).rev() {
            G = r_list[t] + gamma * G;
            let st = s_list[t];
            let at = a_list[t];
            C[(st, at)] = C[(st, at)] + W;
            V[(st, at)] = V[(st, at)] + (W / C[(st, at)]) * (G - V[(st, at)]);
            let argmax = V.slice(s![st, ..]).argmax().unwrap();
            Pi[(st)] = V[(st, argmax)];
            if at as f32 != Pi[st] {
                break;
            }
            W = W * (1.0 / B[(st, at)]);
        }
    }

    (V, Pi)
}
