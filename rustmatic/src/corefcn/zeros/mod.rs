use ndarray::prelude::*;
use ndarray::OwnedRepr;

pub fn zeros(){
    println!("zeros");
}

fn zeros_square_matrix_of_dim(n: usize) -> ArrayBase<OwnedRepr<f64>, Dim<[usize; 2]>> {
    return ndarray::ArrayBase::zeros((n, n));
}

fn zeros_rectangular_matrix_of_dim(m: usize, n: usize) -> ArrayBase<OwnedRepr<f64>, Dim<[usize; 2]>> {
    return ndarray::ArrayBase::zeros((m, n));
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zeros_square_matrix_of_dim_2_works(){
        let a: ArrayBase<ndarray::OwnedRepr<f64>, Dim<[usize; 2]>> = zeros_square_matrix_of_dim(2);
        assert_eq!(array![[0.0,0.0],[0.0,0.0]], a);
    }

    #[test]
    fn zeros_square_matrix_of_dim_3x2_works(){
        let a: ArrayBase<ndarray::OwnedRepr<f64>, Dim<[usize; 2]>> = zeros_rectangular_matrix_of_dim(3, 2);
        assert_eq!(array![[0.0,0.0],[0.0,0.0],[0.0,0.0]], a);
    }
}