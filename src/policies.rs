use ndarray::Array2;

pub fn create_random_uniform_policy(state_size: usize, action_size: usize) -> Array2<f32> {
    Array2::<f32>::ones((state_size, action_size)) / action_size as f32
}
