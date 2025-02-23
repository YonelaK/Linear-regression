use rand::Rng;

pub fn generate_data(size: usize) -> (Vec<f32>, Vec<f32>) {
    let mut x: Vec<f32> = Vec::new();
    let mut y: Vec<f32> = Vec::new();
    let mut rng = rand::thread_rng();

    for _ in 0..size {
        let x_val = rng.gen_range(0.0..100.0);
        let y_val = 2.0 * x_val + 1.0 + rng.gen_range(-10.0..10.0);
        x.push(x_val);
        y.push(y_val);
    }

    (x, y)
}