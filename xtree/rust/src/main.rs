use std::ops::{Add,AddAssign,Sub,SubAssign,Mul,MulAssign,Div,DivAssign,Neg,IndexMut,Index};
#[derive(Copy,Clone,Debug)]
struct Vec3<T> {
    x:T,y:T,z:T
}

impl <T: Add<Output=T>> Add<Vec3<T>> for Vec3<T>{
    type Output = Vec3<T>;
    fn add(self, other:Vec3<T>) -> Vec3<T>{
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z:self.z + other.z
        }
    }
}


impl <T: AddAssign> AddAssign<Vec3<T>> for Vec3<T>{
    fn add_assign(&mut self, other:Vec3<T>){

            self.x += other.x;
            self.y += other.y;
            self.z += other.z;

    }
}

impl <T: Sub<Output=T>> Sub<Vec3<T>> for Vec3<T>{
    type Output = Vec3<T>;
    fn sub(self, other:Vec3<T>) -> Vec3<T>{
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z:self.z - other.z
        }
    }
}


impl <T: SubAssign> SubAssign<Vec3<T>> for Vec3<T>{
    fn sub_assign(&mut self, other:Vec3<T>){

            self.x -= other.x;
            self.y -= other.y;
            self.z -= other.z;

    }
}


impl <T: Mul<Output=T>> Mul<Vec3<T>> for Vec3<T>{
    type Output = Vec3<T>;
    fn mul(self, other:Vec3<T>) -> Vec3<T>{
        Vec3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z:self.z * other.z
        }
    }
}

impl <T: Mul<Output=T> + Copy> Mul<T> for Vec3<T>{
    type Output = Vec3<T>;
    fn mul(self, other:T) -> Vec3<T>{
        Vec3 {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other
        }
    }
}


impl <T: MulAssign> MulAssign<Vec3<T>> for Vec3<T>{
    fn mul_assign(&mut self, other:Vec3<T>){
            self.x *= other.x;
            self.y *= other.y;
            self.z *= other.z;
    }
}

impl <T: MulAssign+Copy> MulAssign<T> for Vec3<T>{
    fn mul_assign(&mut self, other:T){
            self.x *= other;
            self.y *= other;
            self.z *= other;
    }
}

impl <T: Div<Output=T> +  Copy> Div<T> for Vec3<T>{
    type Output = Vec3<T>;
    fn div(self, other:T) -> Vec3<T>{
        Vec3 {
            x: self.x / other,
            y: self.y / other,
            z:self.z / other
        }
    }
}


impl <T: DivAssign + Copy> DivAssign<T> for Vec3<T>{
    fn div_assign(&mut self, other:T){
            self.x /= other;
            self.y /= other;
            self.z /= other;
    }
}

impl<T: Neg<Output=T>> Neg for Vec3<T>{
    type Output = Vec3<T>;
    fn neg(self) -> Vec3<T>{
        Vec3 {
            x : -self.x,
            y : -self.y,
            z : -self.z
        }
    }
}

impl<T: Add<Output=T> + Mul<Output=T> +Neg<Output=T> + Sub<Output = T> + Copy > Vec3<T>{
    fn dot(&self,other:&Vec3<T>) -> T {
        self.x * other.x  + self.y * other.y + self.z * other.z 
    }

    fn length2(&self) -> T{
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    fn cross_product(&self, other:Vec3<T>) -> Vec3<T>{
        Vec3{
            x:  self.y * other.z - self.y * other.z,
            y: -self.x * other.z + self.z * other.x ,
            z:  self.x * other.y - self.y * other.x
        }
    }
}

impl<T> Index<usize> for Vec3<T>{
    type Output = T;
    fn index(&self, idx:usize) -> &T{
        match idx {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Out of Bounds")
        }
    }
} 


impl<'a,T> IndexMut<usize> for Vec3<T>{
    type Output = &'a mut T;
    fn index_mut(&'a mut self, idx:usize) -> &'a mut T{
        match idx {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Out of Bounds")
        }
    }
} 

type Vec3f = Vec3<f64>;


impl Vec3f {
    fn length(&self) -> f64 {
        self.length2().sqrt()
    }

    fn normalize(&mut self) {
        let len = self.length();
        if len > 0.0 {
            self.x /= len;
            self.y /= len;
            self.z /= len;
        }

    }
}
#[derive(Debug)]
struct Matrix33<T>{
    data: Vec3<Vec3<T>>
}


#[derive(Debug)]
struct Matrix44f {
    data: Vec<Vec<f64>>
}

impl Index<usize> for Matrix44f {
    type Output = Vec<f64>;
    fn index(&self, idx: usize) -> &Vec<f64>{
        &self.data[idx]
    }
}

impl IndexMut<usize> for Matrix44f {
    fn index_mut<'a>(&'a mut self, idx: usize) -> &'a mut Vec<f64> {
        // even here I cannot get mutable reference to self.data[idx]
        return self.data.index_mut(idx); 
    }
}

impl Mul<Matrix44f> for Matrix44f{
    type Output = Matrix44f;
    fn mul(self,other:Matrix44f) -> Matrix44f{
        let mut result = Matrix44f::unit();
        for row in 0..4{
            for col in 0..4{
                let mut sum = 0.0;
                for ind in 0..4{
                    sum += self[row][ind] * other[ind][col];
                }
                result[row][col] = sum;
            }
        }
        result
    }
}

impl Matrix44f {
    pub fn unit() -> Matrix44f{
        let mut matrix = Matrix44f{
            data: vec![vec![0.0;4];4]
        };
        for ind in 0..4 {
            matrix.data[ind][ind] =1.0;
        }
        return matrix;
    }

    pub fn new( a:f64,b :f64 ,c:f64 ,d:f64,
                e:f64,f:f64,g:f64,h:f64,
                i:f64,j:f64,k:f64,l:f64,
                m:f64,n:f64,o:f64,p:f64) -> Matrix44f{
        Matrix44f{
            data:vec![
                vec![a,b,c,d],
                vec![e,f,g,h],
                vec![i,j,k,l],
                vec![m,n,o,p]
            ]
        }
    }
}


impl Matrix44f{
    fn transpose(&self) -> Matrix44f{
        Matrix44f::new(self[0][0], self[1][0], self[2][0], self[3][0],
                        self[0][1], self[1][1], self[2][1], self[3][1],
                        self[0][2], self[1][2], self[2][2], self[3][2],
                        self[0][3], self[1][3], self[2][3], self[3][3],)
    }
}

fn main() {
    let x = Vec3 {
        x: 1, y:1 ,z:1
    };
    let y = Vec3f {
        x:1.0,y:1.0,z:1.0
    };
    let mut m = Matrix44f::unit();
    m[1][2] = 2.0;
    println!("m is {:?}, m prime is {:?}",x[0],x[3]);
}
