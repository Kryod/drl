use ndarray::{ s, Array, Array1, Array2, Array3 };
use ndarray_stats::QuantileExt;
use ndarray_rand::{ RandomExt, rand_distr::Uniform };

use crate::{ utils, policies, algorithms };

pub fn policy_iteration(
    S: &Array1<usize>,
    A: &Array1<usize>,
    T: &Array1<usize>,
    P: &Array3<f32>,
    R: &Array3<f32>,
    gamma: Option<f32>,
    theta: Option<f32>,
) -> (Array1<f32>, Array2<f32>) {
    let gamma = gamma.unwrap_or(0.99_f32);
    let theta = theta.unwrap_or(0.000_001_f32);

    let mut V = Array::random(S.shape(), Uniform::new(0.0_f32, 1.0_f32)).into_dimensionality().unwrap();
    V[1] = 0.38;
    V[2] = 0.73;
    V[3] = 0.25;
    utils::apply_for_indices(&mut V, &T, |_idx, x| *x = 0.0);
    let mut Pi = policies::create_random_uniform_policy(S.len(), A.len());

    loop {
        V = algorithms::iterative_policy_evaluation(&S, &A, &T, &P, &R, &Pi, Some(&V), Some(gamma), Some(theta));
        let mut policy_stable = true;

        for s in S.iter() {
            let old_action = Pi.row(*s).argmax().unwrap();

            let mut best_action = 0;
            let mut best_action_score = std::f32::MIN;
            for a in A.iter() {
                let mut temp_score = 0.0;
                for s_p in S.iter() {
                    temp_score += P[(*s, *a, *s_p)] * (R[(*s, *a, *s_p)] + gamma * V[*s_p]);
                }
                if best_action_score <= temp_score {
                    best_action = *a;
                    best_action_score = temp_score;
                }
            }
            Pi.slice_mut(s![*s, ..]).map_inplace(|x| *x = 0.0);
            Pi[(*s, best_action)] = 1.0;
            if best_action != old_action {
                policy_stable = false;
            }
        }

        if policy_stable {
            break;
        }
    }

    (V, Pi)
}
