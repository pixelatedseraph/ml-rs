#![allow(warnings)]
mod matrix;
use matrix::*;
fn main(){
    
    /*let res = mat1.add(&mat2).unwrap_or_else(|err|{
        println!("[{:#?}] Expected dimensions {}x{} but found {}x{}",err,mat1.rows,mat1.cols,mat2.rows,mat2.cols);
        std::process::exit(1);
    }); */

    let mut mat1 = Matrix::fill(3, 2,1.0);
    let mut mat2 = mat1.transpose();

    mat2.scale(25.0);
    
    mat2.print();

}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test_addition_identity(){
        let mat1 = Matrix::fill(1000, 1000, 10.0);
        let mat2 = Matrix::fill(1000, 1000, 0.0);
        let mat3 = mat1.add(&mat2).unwrap();

        assert_eq!(mat3.data,mat1.data);
    }
    #[test]
    fn test_adddition_with_comitativity(){
        let mat1 = Matrix::fill(1000, 1000, 10.0);
        let mat2 = Matrix::fill(1000, 1000, 30.0);

        let mat3 = mat1.add(&mat2).unwrap();
        let mat4 = mat2.add(&mat1).unwrap();

        assert_eq!(mat3.data,mat4.data);
    }
    #[test]
    fn operator_overloading_test(){
        let mat1 = Matrix::fill(10, 10, 100.0);
        let mat2 = Matrix::fill(10, 10, 1000.0);

        let res = mat1 != mat2;
        assert_eq!(res,true)
    }
}