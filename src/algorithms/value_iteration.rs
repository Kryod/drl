use ndarray::{ Array, Array1, Array2, Array3 };
use ndarray_rand::{ RandomExt, rand_distr::Uniform };

use crate::utils;

pub fn value_iteration(
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
    utils::apply_for_indices(&mut V, &T, |_idx, x| *x = 0.0);

    loop {
        let mut delta = 0.0_f32;
        for s in S.iter() {
            let temp_v: f32 = V[*s];
            let mut best = std::f32::MIN;
            for a in A.iter() {
                let mut temp_score = 0.0;
                for s_p in S.iter() {
                    temp_score += P[(*s, *a, *s_p)] * (R[(*s, *a, *s_p)] + gamma * V[*s_p]);
                }
                if best <= temp_score {
                    best = temp_score;
                }
            }
            V[*s] = best;
            delta = delta.max((V[*s] - temp_v).abs());
        }
        if delta < theta {
            break;
        }
    }

    let mut Pi = Array2::zeros((S.len(), A.len()));
    for s in S.iter() {
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
        Pi[(*s, best_action)] = 1.0;
    }

    (V, Pi)
}
