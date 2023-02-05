use ndarray::Array2;
use rand::distributions::StudentT;
use rand::{prelude::Distribution, thread_rng};

pub fn generate_correlated_time_series(
    n_data_points: usize,
    covariance_matrix: Array2<f64>,
    degrees_of_freedom: f64,
) -> Array2<f64> {
    let mut rng = thread_rng();
    let t_dist = StudentT::new(degrees_of_freedom);
    let chol = cholesky(&covariance_matrix);

    let mut uncorrelated_random_vars = Array2::zeros((n_data_points, covariance_matrix.ncols()));
    for i in 0..n_data_points {
        for j in 0..covariance_matrix.ncols() {
            uncorrelated_random_vars[[i, j]] = t_dist.sample(&mut rng);
        }
    }

    uncorrelated_random_vars.dot(&chol.t())
}

fn cholesky(mat: &Array2<f64>) -> Array2<f64> {
    let mut result = Array2::zeros(mat.raw_dim());
    for i in 0..mat.nrows() {
        for j in 0..(i + 1) {
            let mut sum: f64 = 0.0;
            for k in 0..j {
                sum += result[[i, k]] * result[[j, k]];
            }

            if i == j {
                result[[i, j]] = (mat[[i, i]] - sum).sqrt();
            } else {
                result[[i, j]] = 1.0 / result[[j, j]] * (mat[[i, j]] - sum);
            }
        }
    }

    result
}
