#![allow(non_snake_case)]

mod utils;
mod policies;
mod line_world;
mod grid_world;
mod algorithms;
mod stopwatch;

fn main() {
    let mut stopwatch = stopwatch::Stopwatch::new();

    let (S, A, T, P, R) = line_world::init(100);
    let Pi = policies::create_random_uniform_policy(S.len(), A.len());
    let V = algorithms::iterative_policy_evaluation(S, A, T, P, R, Pi, None, None, None);
    println!("Random uniform:");
    dbg!(V);

    println!();

    let (S, A, T, P, R) = line_world::init(100);
    let mut Pi = ndarray::Array2::zeros((S.len(), A.len()));
    utils::apply_for_indices_2(&mut Pi, (&S, &ndarray::arr1(&[1])), |(_a, _b), x| *x = 1.0);
    stopwatch.start();
    let V = algorithms::iterative_policy_evaluation(S, A, T, P, R, Pi, None, None, None);
    stopwatch.stop();
    println!("Stratégie tout à droite");
    dbg!(&V);
    println!("Temps d'exécution: {} secondes", stopwatch.elapsed().unwrap().as_secs_f32());

    println!();

    let (S, A, T, P, R) = grid_world::init(10, 10);
    let Pi = policies::create_random_uniform_policy(S.len(), A.len());
    stopwatch.reset();
    stopwatch.start();
    let V = algorithms::iterative_policy_evaluation(S, A, T, P, R, Pi, None, None, None);
    stopwatch.stop();
    println!("Grid world random uniform:");
    dbg!(&V);
    println!("Temps d'exécution: {} secondes", stopwatch.elapsed().unwrap().as_secs_f32());
}
