#![allow(non_snake_case)]

pub mod utils;
pub mod policies;
pub mod stopwatch;
pub mod line_world;
pub mod grid_world;
pub mod algorithms;

fn main() {
    /*let mut stopwatch = stopwatch::Stopwatch::new();

    let lw = line_world::init(100);
    let Pi = policies::create_random_uniform_policy(lw.S.len(), lw.A.len());
    let V = algorithms::iterative_policy_evaluation(&lw.S, &lw.A, &lw.T, &lw.P, &lw.R, &Pi, None, None, None);
    println!("Random uniform:");
    dbg!(V);

    println!();

    let lw = line_world::init(100);
    let mut Pi = ndarray::Array2::zeros((lw.S.len(), lw.A.len()));
    utils::apply_for_indices_2(&mut Pi, (&lw.S, &ndarray::arr1(&[1])), |(_a, _b), x| *x = 1.0);
    stopwatch.start();
    let V = algorithms::iterative_policy_evaluation(&lw.S, &lw.A, &lw.T, &lw.P, &lw.R, &Pi, None, None, None);
    stopwatch.stop();
    println!("Stratégie tout à droite");
    dbg!(&V);
    println!("Temps d'exécution: {} secondes", stopwatch.elapsed().unwrap().as_secs_f32());

    println!();

    let (S, A, T, P, R) = grid_world::init(10, 10);
    let Pi = policies::create_random_uniform_policy(S.len(), A.len());
    stopwatch.reset();
    stopwatch.start();
    let V = algorithms::iterative_policy_evaluation(&S, &A, &T, &P, &R, &Pi, None, None, None);
    stopwatch.stop();
    println!("Grid world random uniform:");
    dbg!(&V);
    println!("Temps d'exécution: {} secondes", stopwatch.elapsed().unwrap().as_secs_f32());

    println!();*/

    let lw = line_world::init(50);
    let (Q, Pi) = algorithms::monte_carlo_control_with_exploring_starts(lw, line_world::step, line_world::step_until_the_end_of_episode_and_return_transitions, None, Some(5000));
    println!("Action value optimale:\r\n{}", Q);
    println!("Policy optimale:\r\n {}", Pi);
}
