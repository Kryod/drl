use ndarray::Array2;

//def create_random_uniform_policy(state_size: int,
//    action_size: int):
//    return np.ones((state_size, action_size)) / action_size

pub fn create_random_uniform_policy(state_size: usize, action_size: usize) -> Array2<f32> {
    Array2::<f32>::ones((state_size, action_size)) / action_size as f32
}
