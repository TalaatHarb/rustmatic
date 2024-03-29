pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::prelude::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn ndarray_works(){
        let a: ArrayBase<ndarray::OwnedRepr<f64>, Dim<[usize; 2]>> = arr2(&[[1.0,2.0],[3.0,4.0]]);
        let b: ArrayBase<ndarray::OwnedRepr<f64>, Dim<[usize; 2]>> = arr2(&[[5.0,6.0],[7.0,8.0]]);

        let c: ArrayBase<ndarray::OwnedRepr<f64>, Dim<[usize; 2]>> = a.dot(&b);

        assert_eq!(array![[19.0,22.0],[43.0,50.0]], c);
    }
}
