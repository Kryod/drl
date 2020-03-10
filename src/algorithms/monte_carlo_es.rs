use ndarray::{ s, arr1, Array, Array1, Array2, Array3, Ix2 };
use ndarray_rand::{ RandomExt, rand_distr::Uniform };
use ndarray_stats::QuantileExt;
use crate::{ utils, policies, line_world::LineWorld };

/* def monte_carlo_control_with_exploring_starts(
        S: np.ndarray,
        A: np.ndarray,
        T: np.ndarray,
        step_func: Callable,
        step_until_the_end_and_return_transitions_func: Callable,
        gamma: float = 0.99,
        nb_iter = 1000
):
    Pi = create_random_uniform_policy(len(S), len(A))
    Q = np.random.random((len(S), len(A)))
    Q[T, :] = 0.0
    ReturnsSum = np.zeros((len(S), len(A)))
    ReturnsCount = np.zeros((len(S), len(A)))

    for _ in range(nb_iter):
        s0 = np.random.choice(S)

        if s0 in T:
            continue

        a0 = np.random.choice(A)
        r, s = step_func(s0, a0)
        s_list, a_list, r_list, _ = step_until_the_end_and_return_transitions_func(s, Pi)
        G = 0
        s_list = [s0] + s_list
        a_list = [a0] + a_list
        r_list = [r] + r_list
        for t in reversed(range(len(s_list))):
            G = r_list[t] + gamma * G
            st = s_list[t]
            at = a_list[t]
            if s_list[t] in s_list[0:t] and at in a_list[0:t]:
                continue
            ReturnsSum[st, at] += G
            ReturnsCount[st, at] += 1
            Q[st, at] = ReturnsSum[st, at] / ReturnsCount[st, at]
            Pi[st, :] = 0.0
            Pi[st, np.argmax(Q[st])] = 1.0
    return Q, Pi*/

pub fn monte_carlo_control_with_exploring_starts<F, Fi>(
        lw: LineWorld,
        step_func: F,
        step_until_the_end_and_return_transitions_func: Fi,
        gamma: Option<f32>,
        nb_iter: Option<i32>,
    ) -> (Array2<f32>, Array2<f32>)
    where F: Fn(usize, usize, &Array3<f32>, &Array3<f32>, &Array1<usize>) -> (f32, usize), 
        Fi: Fn(usize,
            &Array2<f32>,
            &Array1<usize>,
            &Array1<usize>,
            &Array1<usize>,
            &Array3<f32>,
            &Array3<f32> ) -> (Vec<usize>, Vec<usize>, Vec<f32>, Vec<usize>)
     
    {
    let S = &lw.S;
    let A = &lw.A;
    let T = &lw.T;

    let gamma = gamma.unwrap_or(0.99_f32);
    let nb_iter = nb_iter.unwrap_or(1_000_i32);

    assert!(gamma >= 0.0_f32 && gamma < 1.0_f32);
    assert!(nb_iter > 0_i32);

    let mut Pi = policies::create_random_uniform_policy(S.len(), A.len());
    let mut V = Array::random((S.shape()[0], A.shape()[0]), Uniform::new(0.0_f32, 1.0_f32)).into_dimensionality::<Ix2>().unwrap();
    dbg!(&T);
    V.slice_mut(s![0..T[T.len()-1], ..]).map_inplace(|x| *x = 0.0);
    dbg!(&V);


    let mut returns_sum = Array2::<f32>::zeros((S.len(), A.len()));
    let mut returns_count = Array2::<f32>::zeros((S.len(), A.len()));

    for _ in 0..nb_iter {
        let s0 = utils::rand_pick(&S);

        if utils::contains(&T, s0) {
            continue;
        }
        let a0 = utils::rand_pick(&A);
        let (r, s) = step_func(s0, a0, &lw.P, &lw.R, &lw.S);
        let (mut s_list, mut a_list, mut r_list, _) = step_until_the_end_and_return_transitions_func(s, &Pi, &S, &A, &T,&lw.P,&lw.R);
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
            Pi.slice_mut(s![st, ..]).map_inplace(|x| *x = 0.0);
            let arg = V.slice(s![st, ..]).argmax().unwrap();
            Pi[(st, arg)] = 1.0;
        }
    }

    (V, Pi)
}
