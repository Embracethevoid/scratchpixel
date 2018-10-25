
use std::ops::{Add,AddAssign,Sub,SubAssign,Mul,MulAssign,Div,DivAssign,Neg,IndexMut,Index};
use geometry::vector::*;

use geometry::vector::Vec3;

#[derive(Debug)]
pub struct Matrix33<T>{
    data: Vec3<Vec3<T>>
}


#[derive(Debug,Copy,Clone)]
pub struct Matrix44<T> {
    data: Vec4<Vec4<T>>
}

impl<T> Index<usize> for Matrix44<T> {
    type Output = Vec4<T>;
    fn index(&self, idx: usize) -> &Vec4<T>{
        &self.data[idx]
    }
}

impl<T> IndexMut<usize> for Matrix44<T> {
    fn index_mut<'a>(&'a mut self, idx: usize) -> &'a mut Vec4<T> {
        return self.data.index_mut(idx); 
    }
}

impl<T:Mul<Output=T>+Add<Output=T>+Div<Output = T>+Copy> Mul<Matrix44<T>> for Matrix44<T>{
    type Output = Matrix44<T>;
    fn mul(self,other:Matrix44<T>) -> Matrix44<T>{
        let mut value = Vec::new();
        for row in 0..4{
            for col in 0..4{
                value.push(self[row][0] * other[0][col] + 
                           self[row][1] * other[1][col] + 
                           self[row][2] * other[2][col] + 
                           self[row][3] * other[3][col] 
                        )
            }
        }
        let result = Matrix44::new(
            value[0],
            value[1],
            value[2],
            value[3],
            value[4],
            value[5],
            value[6],
            value[7],
            value[8],
            value[9],
            value[10],
            value[11],
            value[12],
            value[13],
            value[14],
            value[15]
        );
        result
    }
}



impl<T:Copy + Mul<Output=T> + Add<Output=T> + Div<Output = T >> Matrix44<T>{
    pub fn new( a:T,b :T ,c:T ,d:T,
                e:T,f:T,g:T,h:T,
                i:T,j:T,k:T,l:T,
                m:T,n:T,o:T,p:T) -> Matrix44<T>{
        Matrix44{
            data:Vec4{
                x : Vec4{x:a,y:b,z:c,w:d},
                y : Vec4{x:e,y:f,z:g,w:h},
                z : Vec4{x:i,y:j,z:k,w:l},
                w : Vec4{x:m,y:n,z:o,w:p},
            }
        }
    }


    fn transpose(&self) -> Matrix44<T>{
        Matrix44::new(
            self[0][0], self[1][0], self[2][0], self[3][0],
            self[0][1], self[1][1], self[2][1], self[3][1],
            self[0][2], self[1][2], self[2][2], self[3][2],
            self[0][3], self[1][3], self[2][3], self[3][3]
            )
    }

    pub fn mul_vec_matrix(&self , other: &Vec3<T>) -> Vec3<T>{
        let a = other[0] * self[0][0] + other[1] * self[1][0] + other[2] * self[2][0] + self[3][0];
        let b = other[0] * self[0][1] + other[1] * self[1][1] + other[2] * self[2][1] + self[3][1];
        let c = other[0] * self[0][2] + other[1] * self[1][2] + other[2] * self[2][2] + self[3][2];
        let w = other[0] * self[0][3] + other[1] * self[1][3] + other[2] * self[2][3] + self[3][3];

        Vec3{
            x: a/w,
            y: b/w,
            z: c/w
        }
    }

    pub fn mul_dir_matrix(&self , other: Vec3<T>) -> Vec3<T>{
        let a = other[0] * self[0][0] + other[1] * self[1][0] + other[2] * self[2][0] + self[3][0];
        let b = other[0] * self[0][1] + other[1] * self[1][1] + other[2] * self[2][1] + self[3][1];
        let c = other[0] * self[0][2] + other[1] * self[1][2] + other[2] * self[2][2] + self[3][2];

        Vec3{
            x: a,
            y: b,
            z: c
        }
    }
}

pub type Matrix44f = Matrix44<f64>;

impl Matrix44f{
    pub fn unit() -> Matrix44f{
        Matrix44f::new(
        1.0 ,0.0,0.0,0.0,
        0.0 ,1.0,0.0,0.0,
        0.0 ,0.0,1.0,0.0,
        0.0 ,0.0,0.0,1.0
        )
    }
    pub fn inverse(&self) -> Matrix44f{
        let mut s = Matrix44f::unit();
        let mut t = self.clone();
        for i in 0..3{
            let mut pivot = i;
            let mut pivotsize = t[i][i];
            if pivotsize < 0.0 {
                pivotsize = -pivotsize;
            }
            for j in i+1..4{
                let mut tmp = t[j][i];
                if tmp < 0.0 {
                    tmp = -tmp;
                }
                if tmp > pivotsize {
                    pivot = j;
                    pivotsize = tmp;
                }
            }
            if pivotsize == 0.0 {
                return Matrix44f::unit();
            }
            if pivot != i {
                for j in 0..4{
                    let mut tmp = t[i][j];
                    t[i][j] = t[pivot][j];
                    t[pivot][j] = tmp;

                    tmp = s[i][j];
                    s[i][j] = s[pivot][j];
                    s[pivot][j] = tmp;
                }
            }
            for j in i+1..4{
                let f = t[j][i]/t[i][i];
                for k in 0..4{
                    t[j][k] -= f * t[i][k];
                    s[j][k] -= f * s[i][k];
                }
            }
        }

        for i in (0..4).rev(){
            let f = t[i][i];

            if f == 0.0 {
                return Matrix44f::unit();
            }
            for j in 0..4{
                t[i][j] /= f;
                s[i][j] /= f;
            }
            for j in 0..i{
                let g = t[j][i];

                for k in 0..4{
                    t[j][k] -= g * t[i][k];
                    s[j][k] -= g * s[i][k];
                }
            }
        }
        s
    }

 
}
