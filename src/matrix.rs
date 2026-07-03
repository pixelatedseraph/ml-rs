use std::ptr::eq;
use std::{io::empty, ops::*};
use std::ops::*;
use std::clone::Clone;

use rand::*;

#[derive(Debug)]
pub struct Matrix{
    pub rows : usize,
    pub cols : usize,
    pub data : Vec<f64>
}
 #[derive(Debug)]
pub enum MatrixError{
    DimensionMismatch,
}

impl Matrix{
    //empty vector
    pub fn empty(rows: usize,cols: usize) -> Self{
        Matrix { rows, cols ,data: Vec::new() }
    }
    //fills from 1.0 to 10.0
    pub fn random(rows: usize, cols: usize) -> Self {
        let mut mat =  Matrix::empty(rows,cols);

        for _ in 0..rows * cols{
            mat.data.push(rand::random_range(1.0..10.0));
        }    
        mat
    }
    //create a vector with specified size and fill it with a value 
    pub fn fill(rows : usize,cols : usize,value : f64) -> Self{
        let mut mat = Matrix::empty(rows,cols);

        for _ in 0..rows * cols{
            mat.data.push(value);
        } 
        mat
    }
    //empty out the data in the vector
    pub fn clear(&mut self){
        self.data.clear();
    }

    //fill an exisiting vector with some value 
    pub fn fill_existing(&mut self,value : f64) {
        self.clear();

        for _ in 0..self.rows * self.cols{
            self.data.push(value);
        }
    }

    //custom printer for pretty printing a vector
    pub fn print(&self){
        for i in 0..self.rows{
            for j in 0..self.cols{
                print!("{} ",self.at(i,j).unwrap());
            }
            println!("");
        }
        println!("");
    }
    //maps 2d indexing to linear vector indexing and fetches the value there
    pub fn at(&self,row: usize,col: usize) -> Option<f64>{
       let index = self.get_linear_index(row, col)?;
       Some(self.data[index])
    }

    pub fn get_linear_index(&self,row : usize,col : usize) -> Option<usize>{
        if self.rows <= row || self.cols <= col{
            return None;
        }
        Some(row * self.cols + col)
    } 
    
}

impl PartialEq for Matrix{
    fn eq(&self, other: &Self) -> bool {
        if self.rows != other.rows || self.cols != other.cols{
            return false;
        }
        if self.data == other.data{
            return true;
        }
        return false;
    }

    fn ne(&self, other: &Self) -> bool {
        if eq(self, other) == true{
            return false;
        }
        return true;
    }

}

impl Clone for Matrix{
    fn clone(&self) -> Self {
        Self { rows: self.rows, cols: self.cols, data: self.data.clone() }
    }
}


impl Matrix{
    pub fn add(&self,rhs : &Matrix) -> Result<Matrix,MatrixError>{
        if self.rows != rhs.rows || self.cols != rhs.cols{
            return Err(MatrixError::DimensionMismatch);
        }

        let mut res = Matrix::empty(self.rows, self.cols);

        for i in 0..self.rows{
            for j in 0..self.cols{
                res.data.push(self.at(i,j).unwrap() + rhs.at(i,j).unwrap());
            }
        }
        Ok(res)
    }

    pub fn sub(&self,rhs: &Matrix) -> Result<Matrix,MatrixError>{

        if self.rows != rhs.rows || self.cols != rhs.cols{
            return Err(MatrixError::DimensionMismatch);
        }

        let mut res = Matrix::empty(self.rows, self.cols);

        for i in 0..self.rows{
            for j in 0..self.cols{
                res.data.push(self.at(i,j).unwrap() - rhs.at(i,j).unwrap());
            }
        }
        Ok(res)
    }

    pub fn mul(&self,rhs: &Matrix) -> Result<Matrix,MatrixError>{
        if self.cols != rhs.rows {
            return Err(MatrixError::DimensionMismatch);
        }

        let mut res = Matrix::empty(self.rows,rhs.cols); 
        for i in 0..self.rows{
            for j in 0..rhs.cols{
                let mut sum = 0.0;
                for k in 0..self.cols{
                    sum += self.at(i, k).unwrap() * rhs.at(k,j).unwrap();
                }
                res.data.push(sum);
            }
        }
        Ok(res)
    }
    
    pub fn transpose(&self) -> Matrix{
        let mut res = Matrix::empty(self.cols, self.rows);
        res.data = vec![0.0; self.rows * self.cols];

        for i in 0..self.rows{
            for j in 0..self.cols{
                let res_idx = res.get_linear_index(j, i).unwrap();
                let self_idx = self.get_linear_index(i, j).unwrap();
                res.data[res_idx] = self.data[self_idx];
            }
        }
        res
    }

    pub fn scale(&mut self,scalar : f64){
        for i in 0..self.rows{
            for j in 0..self.cols{
                let idx = self.get_linear_index(i, j).unwrap();
                self.data[idx] = scalar;
            }
        }
    }

}




/* pub fn generate_seed() -> Result<u64,Box<dyn Error>>{
    let mut file_handle= File::open("/dev/urandom")?;
    let mut string_buffer = [0u8; 8];
    file_handle.read_exact(&mut string_buffer)?;

    Ok(u64::from_ne_bytes(string_buffer))
} */