pub struct LinearRegression {
    w: f32,
    b: f32,
}

impl LinearRegression {
    pub fn new() -> Self {
        LinearRegression {
            w: 0.0,
            b: 0.0,
        }
    }

    pub fn train(&mut self, x: Vec<f32>, y: Vec<f32>, learning_rate: f32, iterations: usize) {
        for _ in 0..iterations {
            let mut w_gradient = 0.0;
            let mut b_gradient = 0.0;

            for i in 0..x.len() {
                let prediction = self.predict(x[i]);
                w_gradient += (prediction - y[i]) * x[i];
                b_gradient += prediction - y[i];
            }

            self.w -= learning_rate * (w_gradient / x.len() as f32);
            self.b -= learning_rate * (b_gradient / x.len() as f32);
        }
    }

    pub fn predict(&self, x: f32) -> f32 {
        self.w * x + self.b
    }

    pub fn loss(&self, x: Vec<f32>, y: Vec<f32>) -> f32 {
        let mut loss = 0.0;
        for i in 0..x.len() {
            let prediction = self.predict(x[i]);
            loss += (prediction - y[i]).powi(2);
        }
        loss / (x.len() as f32)
    }
}