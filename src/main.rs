mod correlated_time_series;
mod csv_writer;
use correlated_time_series::generate_correlated_time_series;
use ndarray::prelude::*;

fn main() {
    let n_data_points = 1000;
    let covariance_matrix: ndarray::Array2<f64> = array![[1.0, 0.8], [0.8, 1.0]];
    let data = generate_correlated_time_series(n_data_points, covariance_matrix, 4.0);

    csv_writer::write_to_csv("test_data.csv", &data).unwrap();
}
