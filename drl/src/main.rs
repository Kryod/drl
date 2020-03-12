#![allow(non_snake_case)]

pub mod utils;
pub mod policies;
pub mod stopwatch;
pub mod line_world;
pub mod grid_world;
pub mod algorithms;

fn main() {
    /*
    println!("---------------ITERATIVE POLICY EVALUATION------------------");
    let mut stopwatch = stopwatch::Stopwatch::new();

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

    let gw = grid_world::init(10, 10);
    let Pi = policies::create_random_uniform_policy(gw.S.len(), gw.A.len());
    stopwatch.reset();
    stopwatch.start();
    let V = algorithms::iterative_policy_evaluation(&lw.S, &lw.A, &lw.T, &lw.P, &lw.R, &Pi, None, None, None);
    stopwatch.stop();
    println!("Grid world random uniform:");
    dbg!(&V);
    println!("Temps d'exécution: {} secondes", stopwatch.elapsed().unwrap().as_secs_f32());

    println!();*/

    /*
    println!("---------------VALUE ITERATION------------------");
    let mut stopwatch = stopwatch::Stopwatch::new();

    let lw = line_world::init(100);
    let V = algorithms::value_iteration(&lw.S, &lw.A, &lw.T, &lw.P, &lw.R, None, None);
    println!("Random uniform:");
    dbg!(V);

    println!();
    let gw = grid_world::init(10, 10);
    stopwatch.reset();
    stopwatch.start();
    let V = algorithms::value_iteration(&gw.S, &gw.A, &gw.T, &gw.P, &gw.R, None, None);
    stopwatch.stop();
    println!("Grid world random uniform:");
    dbg!(&V);
    println!("Temps d'exécution: {} secondes", stopwatch.elapsed().unwrap().as_secs_f32());

    println!();
    */

    /*
    println!("---------------POLICY ITERATION------------------");
    let mut stopwatch = stopwatch::Stopwatch::new();

    let lw = line_world::init(100);
    let V = algorithms::value_iteration(&lw.S, &lw.A, &lw.T, &lw.P, &lw.R, None, None);
    println!("Random uniform:");
    dbg!(V);

    println!();
    let gw = grid_world::init(10, 10);
    stopwatch.reset();
    stopwatch.start();
    let V = algorithms::value_iteration(&gw.S, &gw.A, &gw.T, &gw.P, &gw.R, None, None);
    stopwatch.stop();
    println!("Grid world random uniform:");
    dbg!(&V);
    println!("Temps d'exécution: {} secondes", stopwatch.elapsed().unwrap().as_secs_f32());

    println!();
    */

    
    /*println!("---------------MONTE CARLO ES------------------");
    let lw = line_world::init(5);
    let (Q, Pi) = algorithms::monte_carlo_control_with_exploring_starts(&lw, line_world::step, line_world::step_until_the_end_of_episode_and_return_transitions, None, Some(5000));
    println!("Action value optimale:\r\n{}", Q);
    println!("Policy optimale:\r\n {}", Pi);

    let lw = grid_world::init(2, 2);
    let (Q, Pi) = algorithms::monte_carlo_control_with_exploring_starts(&lw, line_world::step, line_world::step_until_the_end_of_episode_and_return_transitions, None, Some(5000));
    println!("Action value optimale:\r\n{}", Q);
    println!("Policy optimale:\r\n {}", Pi);*/

    /*println!("---------------MONTE CARLO ON-POLICY------------------");
    let lw = line_world::init(5);
    let (Q, Pi) = algorithms::monte_carlo_control_on_policy(&lw, line_world::step, line_world::step_until_the_end_of_episode_and_return_transitions, None, Some(5000), None);
    println!("Action value optimale:\r\n{}", Q);
    println!("Policy optimale:\r\n {}", Pi);

    let lw = grid_world::init(2, 2);
    let (Q, Pi) = algorithms::monte_carlo_control_on_policy(&lw, line_world::step, line_world::step_until_the_end_of_episode_and_return_transitions, None, Some(5000), None);
    println!("Action value optimale:\r\n{}", Q);
    println!("Policy optimale:\r\n {}", Pi);*/
    
    /*println!("---------------MONTE CARLO OFF-POLICY------------------");
    let lw = line_world::init(5);
    let (Q, Pi) = algorithms::monte_carlo_control_off_policy(&lw, line_world::step, line_world::step_until_the_end_of_episode_and_return_transitions, None, Some(5000));
    println!("Action value optimale:\r\n{}", Q);
    println!("Policy optimale:\r\n {}", Pi);

    let lw = grid_world::init(2, 2);
    let (Q, Pi) = algorithms::monte_carlo_control_off_policy(&lw, line_world::step, line_world::step_until_the_end_of_episode_and_return_transitions, None, Some(5000));
    println!("Action value optimale:\r\n{}", Q);
    println!("Policy optimale:\r\n {}", Pi);*/

    /*println!("---------------SARSA------------------");
    let lw = line_world::init(5);
    let Q = algorithms::sarsa(&lw, line_world::step, None, Some(5000), None, None);
    println!("Action value optimale:\r\n{}", Q);
    //println!("Policy optimale:\r\n {}", Pi);

    let lw = grid_world::init(2, 2);
    let Q = algorithms::sarsa(&lw, line_world::step, None, Some(5000), None, None);
    println!("Action value optimale:\r\n{}", Q);
    //println!("Policy optimale:\r\n {}", Pi);*/

    // state = {2, 2, 2, 2, 2, 2, 2, 2, 2}
    //Total[Table[Power[3, (i-1)]*state[[i]], {i, 1, Length[state]}]]
    
    println!("---------------DYNA-Q------------------");
    let lw = line_world::init(5);
    let Q = algorithms::dyna_q(&lw, line_world::step, None, Some(5000), Some(2), None);
    println!("Action value optimale:\r\n{}", Q);
    //println!("Policy optimale:\r\n {}", Pi);

    let lw = grid_world::init(3, 3);
    let Q = algorithms::dyna_q(&lw, line_world::step, None, Some(1000), Some(10), None);
    println!("Action value optimale:\r\n{}", Q);
    //println!("Policy optimale:\r\n {}", Pi);
}
