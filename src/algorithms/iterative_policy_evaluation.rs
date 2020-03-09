use ndarray::{ Array, Array1, Array2, Array3 };
use ndarray_rand::{ RandomExt, rand_distr::Uniform };

pub fn iterative_policy_evaluation(
    S: Array1<usize>,
    A: Array1<usize>,
    T: Array1<usize>,
    P: Array3<f32>,
    R: Array3<f32>,
    Pi: Array2<f32>,
    V: Option<Array1<f32>>,
    gamma: Option<f32>,
    theta: Option<f32>,
) -> Array1<f32> {
    let gamma = gamma.unwrap_or(0.99_f32);
    let theta = theta.unwrap_or(0.000_001_f32);

    assert!(gamma >= 0.0_f32 && gamma < 1.0_f32);
    assert!(theta > 0.0_f32);

    let mut V = V.unwrap_or(Array::random(S.shape(), Uniform::new(0.0_f32, 1.0_f32)).into_dimensionality().unwrap());
    crate::utils::apply_for_indices(&mut V, &T, |_idx, x| *x = 0.0_f32);

    loop {
        let mut delta = 0.0_f32;
        for s in S.iter() {
            let temp_v: f32 = V[*s];
            let mut temp_sum = 0.0_f32;
            for a in A.iter() {
                for s_p in S.iter() {
                    temp_sum += Pi[(*s, *a)] * P[(*s, *a, *s_p)] * (
                        R[(*s, *a, *s_p)] + gamma * V[*s_p]
                    );
                }
            }
            V[*s] = temp_sum;
            delta = delta.max((V[*s] - temp_v).abs());
        }
        if delta < theta {
            break;
        }
    }

    V
}
