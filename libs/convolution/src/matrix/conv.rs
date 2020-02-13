use simple_matrix::Matrix;

pub trait Convolution<T> {
    type Output;

    fn conv(self, rhs: Matrix<T>) -> Self::Output;
}

impl<T> Convolution<T> for Matrix<T>
where
    T: Convolution<Output = T>
{
    type Output = Matrix<T>;
    
    fn conv(self, rhs: Matrix<T>) -> Self::Output {
        Matrix::new(5, 5)
    }
}
