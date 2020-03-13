use ndarray::{ s, arr1, Array, Array2, Ix2 };
use ndarray_rand::{ RandomExt, rand_distr::Uniform };
use ndarray_stats::QuantileExt;
use crate::{ utils, policies, grid_world::World, inc_vec };

pub fn monte_carlo_control_on_policy(
        w: &impl World,
        gamma: Option<f32>,
        nb_iter: Option<i32>,
        epsilon: Option<f32>
    ) -> (Array2<f32>, Array2<f32>) {

    let (S, A, T, P, R) = w.get_all();

    let gamma = gamma.unwrap_or(0.99_f32);
    let nb_iter = nb_iter.unwrap_or(1_000_i32);
    let epsilon = epsilon.unwrap_or(0.1_f32);

    assert!(gamma >= 0.0_f32 && gamma < 1.0_f32);
    assert!(nb_iter > 0_i32);

    let mut Pi = policies::create_random_uniform_policy(S.len(), A.len());
    let mut V = Array::random((S.shape()[0], A.shape()[0]), Uniform::new(0.0_f32, 1.0_f32)).into_dimensionality::<Ix2>().unwrap();
    let shape = V.shape()[1];
    utils::apply_for_indices_2(&mut V, (T, &arr1(&inc_vec![shape]) ), |_idx, x| *x = 0.0);


    let mut returns_sum = Array2::<f32>::zeros((S.len(), A.len()));
    let mut returns_count = Array2::<f32>::zeros((S.len(), A.len()));

    for _ in 0..nb_iter {
        let s0 = utils::rand_pick(&S);

        if utils::contains(&T, s0) {
            continue;
        }
        let a0 = utils::rand_pick(&A);
        let (r, s) = w.step(s0, a0, &P, &R, &S);
        let (mut s_list, mut a_list, mut r_list, _) = w.step_until_the_end_of_episode_and_return_transitions(s, &Pi, &S, &A, &T,&P,&R);
        let mut G = 0.0;
        s_list.insert(0, s0);
        a_list.insert(0, a0);
        r_list.insert(0, r);
        for t in (0..s_list.len()).rev() {
            G = r_list[t] + gamma * G;
            let st = s_list[t];
            let at = a_list[t];
            if utils::contains(&arr1(&s_list[0..t]), s_list[t]) && utils::contains(&arr1(&a_list[0..t]), at) {
                continue
            }
            returns_sum[(st, at)] += G;
            returns_count[(st, at)] += 1.0;
            V[(st, at)] = returns_sum[(st, at)] / returns_count[(st, at)];
            let index = V.slice(s![st, ..]).argmax().unwrap();
            for a in 0..V.shape()[1] {
                if a == index {
                    Pi[(st, a)] = 1.0 - epsilon + (epsilon / (V.slice(s![st, ..]).sum().abs()) as f32)
                } else {
                    Pi[(st, a)] = epsilon / (V.slice(s![st, ..]).sum().abs()) as f32;
                }
            }
        }
    }

    (V, Pi)
}
