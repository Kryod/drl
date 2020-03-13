#![allow(non_snake_case)]

use ffi_csharp::ffi;
use ndarray::{ Array1, Array2 };

pub mod tools;
pub mod utils;
pub mod policies;
pub mod stopwatch;
pub mod line_world;
pub mod grid_world;
pub mod algorithms;

use grid_world::World;

#[ffi("drl", "unity-drl")]
pub enum Algorithm {
    PolicyEvaluation,
    PolicyIteration,
    ValueIteration,
    MonteCarloExploringStarts,
    MonteCarloOnPolicyFirstVisit,
    MonteCarloOffPolicy,
    Sarsa,
    QLearning,
    ExpectedSarsa,
    DynaQ,
    EpisodicSemiGradiantSarsa,
    DeepQLearning,
    Reinforce,
    ReinforceWithBaseline,
    OneStepActorCritic,
    PpoA2cStyle,
}

#[derive(Default)]
pub struct Drl {
    lw: Option<Box<dyn World>>,
    gw: Option<Box<dyn World>>,

    gamma: Option<f32>,
    theta: Option<f32>,
    epsilon: Option<f32>,
    alpha: Option<f32>,
    nb_iter: Option<i32>,
    max_step: Option<i32>,
    n: Option<i32>,

    Pi: Option<Array2<f32>>,
    V: Option<Array1<f32>>,
    Q: Option<Array2<f32>>,
}

impl Drl {
    pub fn new() -> Self {
        Self::default()
    }
}

#[ffi("drl", "unity-drl")]
impl Drl {
    #[no_mangle]
    pub fn init_line_world(&mut self, num_states: usize) {
        self.lw = Some(Box::new(line_world::init(num_states)));
        self.gw = None;
    }

    #[no_mangle]
    pub fn init_grid_world(&mut self, width: usize, height: usize) {
        self.lw = None;
        self.gw = Some(Box::new(grid_world::init(width, height)));
    }

    #[no_mangle]
    pub fn run(&mut self, alg: Algorithm) {
        let mut w = None;
        if let Some(ref lw) = self.lw {
            w = Some(lw);
        };
        if let Some(ref gw) = self.gw {
            w = Some(gw);
        };
        let w = match w {
            Some(w) => w,
            None => return,
        };

        let (S, A, T, P, R) = w.get_all();
        match alg {
            Algorithm::PolicyEvaluation => {
                let Pi = policies::create_random_uniform_policy(S.len(), A.len());
                self.V = Some(algorithms::iterative_policy_evaluation(&S, &A, &T, &P, &R, &Pi, None, self.gamma, self.theta));
                self.Pi = Some(Pi);
            },
            Algorithm::PolicyIteration => {
                let (V, Pi) = algorithms::policy_iteration(&S, &A, &T, &P, &R, self.gamma, self.theta);
                self.V = Some(V);
                self.Pi = Some(Pi);
            },
            Algorithm::ValueIteration => {
                let (V, Pi) = algorithms::value_iteration(&S, &A, &T, &P, &R, self.gamma, self.theta);
                self.V = Some(V);
                self.Pi = Some(Pi);
            },
            Algorithm::MonteCarloExploringStarts => {
                let (Q, Pi) = algorithms::monte_carlo_control_with_exploring_starts(w, self.gamma, self.nb_iter);
                self.Q = Some(Q);
                self.Pi = Some(Pi);
            },
            Algorithm::MonteCarloOnPolicyFirstVisit => {
                let (Q, Pi) = algorithms::monte_carlo_control_on_policy(w, self.gamma, self.nb_iter, self.epsilon);
                self.Q = Some(Q);
                self.Pi = Some(Pi);
            },
            Algorithm::MonteCarloOffPolicy => {
                let (Q, _Pi) = algorithms::monte_carlo_control_off_policy(w, self.gamma, self.nb_iter);
                self.Q = Some(Q);
                //self.Pi = Some(Pi);
            },
            Algorithm::Sarsa => {
                self.Q = Some(algorithms::sarsa(w, self.gamma, self.nb_iter, self.max_step, self.epsilon, self.alpha));
            },
            Algorithm::QLearning => {
                self.Q = Some(algorithms::q_learning(w, self.gamma, self.nb_iter, self.max_step, self.epsilon, self.alpha));
            },
            Algorithm::ExpectedSarsa => {

            },
            Algorithm::DynaQ => {
                self.Q = Some(algorithms::dyna_q(w, self.gamma, self.nb_iter, self.n, self.epsilon, self.alpha));
            },
            Algorithm::EpisodicSemiGradiantSarsa => {

            },
            Algorithm::DeepQLearning => {

            },
            Algorithm::Reinforce => {

            },
            Algorithm::ReinforceWithBaseline => {

            },
            Algorithm::OneStepActorCritic => {

            },
            Algorithm::PpoA2cStyle => {

            },
        };
    }

    #[no_mangle]
    pub fn set_gamma(&mut self, gamma: f32) {
        self.gamma = Some(gamma);
    }

    #[no_mangle]
    pub fn set_theta(&mut self, theta: f32) {
        self.theta = Some(theta);
    }

    #[no_mangle]
    pub fn set_epsilon(&mut self, epsilon: f32) {
        self.epsilon = Some(epsilon);
    }

    #[no_mangle]
    pub fn set_alpha(&mut self, alpha: f32) {
        self.alpha = Some(alpha);
    }

    #[no_mangle]
    pub fn set_nb_iter(&mut self, nb_iter: i32) {
        self.nb_iter = Some(nb_iter);
    }

    #[no_mangle]
    pub fn set_max_step(&mut self, max_step: i32) {
        self.max_step = Some(max_step);
    }

    #[no_mangle]
    pub fn set_n(&mut self, n: i32) {
        self.n = Some(n);
    }

    #[no_mangle]
    pub unsafe fn get_v(&self, len: *mut usize) -> *const f32 {
        match &self.V {
            Some(V) => {
                *len = V.len();
                V.as_ptr()
            },
            None => {
                *len = 0;
                std::ptr::null()
            },
        }
    }

    #[no_mangle]
    pub unsafe fn get_q(&self, len: *mut usize, n_actions: *mut usize) -> *const f32 {
        match &self.Q {
            Some(Q) => {
                *len = Q.len();
                *n_actions = Q.shape()[1];
                Q.as_ptr()
            }
            None => {
                *len = 0;
                std::ptr::null()
            },
        }
    }

    #[no_mangle]
    pub unsafe fn get_pi(&self, len: *mut usize, n_actions: *mut usize) -> *const f32 {
        match &self.Pi {
            Some(Pi) => {
                *len = Pi.len();
                *n_actions = Pi.shape()[1];
                Pi.as_ptr()
            }
            None => {
                *len = 0;
                std::ptr::null()
            },
        }
    }
}
