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
        let a = arr2(&[[1,2],[3,4]]);
        let b = arr2(&[[5,6],[7,8]]);

        let c = a.dot(&b);

        assert_eq!(array![[19,22],[43,50]], c);
    }
}
