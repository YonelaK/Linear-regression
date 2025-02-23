mod model;
mod data;

use model::LinearRegression;
use data::generate_data;
use rand::Rng;
use textplots::{Chart, Plot, Shape};

fn main() {
    let (x, y) = generate_data(100);

    println!("Original Data:");
    for (x, y) in x.iter().zip(y.iter()) {
        println!("x: {:.5}, y: {:.5}", x, y);
    }

    let mut model = LinearRegression::new();
    let mut loss = f32::INFINITY;
    for i in 0..1000 {
        model.train(x.clone(), y.clone(), 0.01, 1);
        let new_loss = model.loss(x.clone(), y.clone());
        if !new_loss.is_nan() {
            println!("Iteration {}: Loss = {}", i, new_loss);
            if new_loss < loss {
                loss = new_loss;
            }
        }
    }

    let predictions: Vec<f32> = x.iter().map(|x| model.predict(*x)).collect();
    let mse = predictions.iter().zip(y.iter()).map(|(p, y)| (p - y).powi(2)).sum::<f32>() / predictions.len() as f32;
    println!("MSE: {}", mse);

    println!("Predictions:");
    for ((x, y), prediction) in x.iter().zip(y.iter()).zip(predictions.iter()) {
        println!("x: {:.5}, y: {:.5}, prediction: {:.5}", x, y, prediction);
    }

    let pairs: Vec<(f32, f32)> = x.iter().zip(y.iter()).map(|(x, y)| (*x, *y)).collect();
    let prediction_pairs: Vec<(f32, f32)> = x.iter().zip(predictions.iter()).map(|(x, y)| (*x, *y)).collect();

    let pairs_shape = Shape::Lines(pairs.as_slice());
    let prediction_pairs_shape = Shape::Lines(prediction_pairs.as_slice());

    let mut binding = Chart::default();
    let chart = binding
        .lineplot(&pairs_shape)
        .lineplot(&prediction_pairs_shape);

    println!("Graph:");
    chart.display();
}