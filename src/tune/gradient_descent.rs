use log::trace;
use rayon::prelude::*;
use std::time::Instant;

pub fn gradient_descent<const N: usize>(
    coefficient_sets: &[[f32; N]],
    results: &[f32],
    test_coefficient_sets: &[[f32; N]],
    test_results: &[f32],
    params: &[f32; N],
    initial_learning_rate: f32,
) -> [f32; N] {
    assert_eq!(coefficient_sets.len(), results.len());
    assert_eq!(test_coefficient_sets.len(), test_results.len());

    let start_time = Instant::now();
    let beta = 0.95;

    // If error is not reduced this number of times, reduce eta, or abort if eta is already low
    const MAX_TRIES: usize = 100;

    let initial_error = average_error(test_coefficient_sets, test_results, params);
    println!(
        "Running gradient descent on {} positions and {} test positions",
        coefficient_sets.len(),
        test_coefficient_sets.len()
    );
    println!("Initial parameters: {:?}", params);
    println!("Initial test error: {}", initial_error);
    println!(
        "Initial training error: {}",
        average_error(coefficient_sets, results, params)
    );

    let mut lowest_error = initial_error;
    let mut best_parameter_set = *params;

    for eta in [
        initial_learning_rate,
        initial_learning_rate / 3.0,
        initial_learning_rate / 10.0,
        initial_learning_rate / 30.0,
    ]
    .iter()
    {
        trace!("\nTuning with eta = {}\n", eta);
        let mut parameter_set = best_parameter_set;
        let mut gradients = [0.0; N];

        let mut iterations_since_improvement = 0;
        let mut iterations_since_large_improvement = 0;
        loop {
            let slopes = calc_slope(coefficient_sets, results, &parameter_set);
            trace!("Slopes: {:?}", slopes);
            gradients
                .iter_mut()
                .zip(slopes.iter())
                .for_each(|(gradient, slope)| *gradient = beta * *gradient + (1.0 - beta) * slope);
            trace!("Gradients: {:?}", gradients);

            parameter_set
                .iter_mut()
                .zip(gradients.iter())
                .for_each(|(param, gradient)| *param -= gradient * eta);
            trace!("New parameters: {:?}", parameter_set);

            let error = average_error(test_coefficient_sets, test_results, &parameter_set);
            trace!("Error now {}, eta={}\n", error, eta);

            if error < lowest_error {
                iterations_since_improvement = 0;
                if lowest_error / error > 1.000_001 {
                    iterations_since_large_improvement = 0;
                } else {
                    iterations_since_large_improvement += 1;
                    if iterations_since_large_improvement >= MAX_TRIES {
                        break;
                    }
                }
                lowest_error = error;
                best_parameter_set = parameter_set;
            } else {
                iterations_since_improvement += 1;
                iterations_since_large_improvement += 1;
                if iterations_since_improvement >= MAX_TRIES {
                    break;
                }
            }
        }
    }

    let elapsed = start_time.elapsed();

    println!(
        "Finished gradient descent in {:.1}s, error is {:.7}. Parameters:\n{:?}",
        elapsed.as_secs_f64(),
        lowest_error,
        best_parameter_set
            .iter()
            .map(|f| *f as f32)
            .collect::<Vec<f32>>()
    );
    best_parameter_set
}

/// For each parameter, calculate the slope for that dimension
fn calc_slope<const N: usize>(
    coefficient_sets: &[[f32; N]],
    results: &[f32],
    params: &[f32; N],
) -> [f32; N] {
    let mut slopes = coefficient_sets
        .par_iter()
        .zip(results)
        .map(|(coefficients, result)| {
            let estimated_result = eval_from_params(coefficients, params);
            let estimated_sigmoid = sigmoid(estimated_result);
            let derived_sigmoid_result = sigmoid_derived(estimated_result);

            let mut gradients_for_this_training_sample = [0.0; N];
            gradients_for_this_training_sample
                .iter_mut()
                .zip(coefficients)
                .for_each(|(gradient, coefficient)| {
                    *gradient = (estimated_sigmoid - result) * derived_sigmoid_result * *coefficient
                });
            gradients_for_this_training_sample
        })
        // Sum each individual chunk as f32
        // Then sum those chunks as f64, to avoid rounding errors
        .chunks(256)
        .map(|chunks: Vec<[f32; N]>| {
            chunks.into_iter().fold([0.0; N], |mut a, b| {
                for (c, d) in a.iter_mut().zip(b.iter()) {
                    *c += *d;
                }
                a
            })
        })
        .fold(
            || [0.0; N],
            |mut a, b| {
                for (c, d) in a.iter_mut().zip(b.iter()) {
                    *c += *d as f64;
                }
                a
            },
        )
        .reduce(
            || [0.0; N],
            |mut a, b| {
                for (c, d) in a.iter_mut().zip(b.iter()) {
                    *c += *d;
                }
                a
            },
        );

    for slope in slopes.iter_mut() {
        *slope /= coefficient_sets.len() as f64;
    }
    let mut f32_slopes = [0.0; N];
    for (f64_slope, slope) in f32_slopes.iter_mut().zip(&slopes) {
        *f64_slope = *slope as f32;
    }
    f32_slopes
}

/// Mean squared error of the parameter set, measured against given results and positions
fn average_error<const N: usize>(
    coefficient_sets: &[[f32; N]],
    results: &[f32],
    params: &[f32; N],
) -> f32 {
    assert_eq!(coefficient_sets.len(), results.len());
    coefficient_sets
        .into_par_iter()
        .zip(results)
        .map(|(coefficients, game_result)| {
            (sigmoid(eval_from_params(coefficients, params)) - game_result).powf(2.0)
        })
        .map(|f| f as f64)
        .sum::<f64>() as f32
        / (coefficient_sets.len() as f32)
}

pub fn eval_from_params<const N: usize>(coefficients: &[f32; N], params: &[f32; N]) -> f32 {
    coefficients.iter().zip(params).map(|(c, p)| c * p).sum()
}

pub fn sigmoid(x: f32) -> f32 {
    1.0 / (1.0 + f32::exp(-x as f32))
}

pub fn sigmoid_derived(x: f32) -> f32 {
    f32::exp(x) / f32::powi(1.0 + f32::exp(x), 2)
}
