#![allow(non_snake_case)]

mod utils;
mod policies;
mod line_world;
mod algorithms;

fn main() {
    let (S, A, T, P, R) = line_world::init();
    let Pi = policies::create_random_uniform_policy(S.len(), A.len());
    let V = algorithms::iterative_policy_evaluation(S, A, T, P, R, Pi, None, None, None);
    println!("Random uniform:");
    dbg!(V);

    let (S, A, T, P, R) = line_world::init();
    let mut Pi = ndarray::Array2::zeros((S.len(), A.len()));
    utils::apply_for_indices_2(&mut Pi, (&S, &ndarray::arr1(&[1])), |(_a, _b), x| *x = 1.0);

    let V = algorithms::iterative_policy_evaluation(S, A, T, P, R, Pi, None, None, None);
    println!("Stratégie tout à droite");
    dbg!(&V);
}
