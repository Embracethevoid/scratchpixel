use std::ops::{Add,AddAssign,Sub,SubAssign,Mul,MulAssign,Div,DivAssign,Neg,IndexMut,Index};
#[derive(Copy,Clone,Debug)]
struct Vec2<T> {
    x:T,y:T
}

impl <T: Add<Output=T>> Add<Vec2<T>> for Vec2<T>{
    type Output = Vec2<T>;
    fn add(self, other:Vec2<T>) -> Vec2<T>{
        Vec2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}


impl <T: AddAssign> AddAssign<Vec2<T>> for Vec2<T>{
    fn add_assign(&mut self, other:Vec2<T>){

            self.x += other.x;
            self.y += other.y;

    }
}

impl <T: Sub<Output=T>> Sub<Vec2<T>> for Vec2<T>{
    type Output = Vec2<T>;
    fn sub(self, other:Vec2<T>) -> Vec2<T>{
        Vec2 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}


impl <T: SubAssign> SubAssign<Vec2<T>> for Vec2<T>{
    fn sub_assign(&mut self, other:Vec2<T>){

            self.x -= other.x;
            self.y -= other.y;

    }
}


impl <T: Mul<Output=T>> Mul<Vec2<T>> for Vec2<T>{
    type Output = Vec2<T>;
    fn mul(self, other:Vec2<T>) -> Vec2<T>{
        Vec2 {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }
}

impl <T: Mul<Output=T> + Copy> Mul<T> for Vec2<T>{
    type Output = Vec2<T>;
    fn mul(self, other:T) -> Vec2<T>{
        Vec2 {
            x: self.x * other,
            y: self.y * other,
        }
    }
}


impl <T: MulAssign> MulAssign<Vec2<T>> for Vec2<T>{
    fn mul_assign(&mut self, other:Vec2<T>){
            self.x *= other.x;
            self.y *= other.y;
    }
}

impl <T: MulAssign+Copy> MulAssign<T> for Vec2<T>{
    fn mul_assign(&mut self, other:T){
            self.x *= other;
            self.y *= other;
    }
}

impl <T: Div<Output=T> +  Copy> Div<T> for Vec2<T>{
    type Output = Vec2<T>;
    fn div(self, other:T) -> Vec2<T>{
        Vec2 {
            x: self.x / other,
            y: self.y / other,
        }
    }
}


impl <T: DivAssign + Copy> DivAssign<T> for Vec2<T>{
    fn div_assign(&mut self, other:T){
            self.x /= other;
            self.y /= other;
    }
}

impl<T: Neg<Output=T>> Neg for Vec2<T>{
    type Output = Vec2<T>;
    fn neg(self) -> Vec2<T>{
        Vec2 {
            x : -self.x,
            y : -self.y,
        }
    }
}

impl<T: Add<Output=T> + Mul<Output=T> +Neg<Output=T> + Sub<Output = T> + Copy > Vec2<T>{
    fn dot(&self,other:&Vec2<T>) -> T {
        self.x * other.x  + self.y * other.y
    }

    fn length2(&self) -> T{
        self.x * self.x + self.y * self.y 
    }

}

impl<T> Index<usize> for Vec2<T>{
    type Output = T;
    fn index(&self, idx:usize) -> &T{
        match idx {
            0 => &self.x,
            1 => &self.y,
            _ => panic!("Out of Bounds")
        }
    }
} 


impl<T> IndexMut<usize> for Vec2<T>{
    fn index_mut<'a>(&'a mut self, idx:usize) -> &'a mut T{
        match idx {
            0 => &mut self.x,
            1 => &mut self.y,
            _ => panic!("Out of Bounds")
        }
    }
} 









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


impl<T> IndexMut<usize> for Vec3<T>{
    fn index_mut<'a>(&'a mut self, idx:usize) -> &'a mut T{
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



/*******************************
Vec4
**********************************/

#[derive(Copy,Clone,Debug)]
struct Vec4<T> {
    x:T,y:T,z:T,w:T
}

impl <T: Add<Output=T>> Add<Vec4<T>> for Vec4<T>{
    type Output = Vec4<T>;
    fn add(self, other:Vec4<T>) -> Vec4<T>{
        Vec4 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w
        }
    }
}


impl <T: AddAssign> AddAssign<Vec4<T>> for Vec4<T>{
    fn add_assign(&mut self, other:Vec4<T>){

            self.x += other.x;
            self.y += other.y;
            self.z += other.z;
            self.w += other.w;

    }
}

impl <T: Sub<Output=T>> Sub<Vec4<T>> for Vec4<T>{
    type Output = Vec4<T>;
    fn sub(self, other:Vec4<T>) -> Vec4<T>{
        Vec4 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w
        }
    }
}


impl <T: SubAssign> SubAssign<Vec4<T>> for Vec4<T>{
    fn sub_assign(&mut self, other:Vec4<T>){

            self.x -= other.x;
            self.y -= other.y;
            self.z -= other.z;
            self.w -= other.w;

    }
}


impl <T: Mul<Output=T>> Mul<Vec4<T>> for Vec4<T>{
    type Output = Vec4<T>;
    fn mul(self, other:Vec4<T>) -> Vec4<T>{
        Vec4 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
            w: self.w * other.w
        }
    }
}

impl <T: Mul<Output=T> + Copy> Mul<T> for Vec4<T>{
    type Output = Vec4<T>;
    fn mul(self, other:T) -> Vec4<T>{
        Vec4 {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
            w: self.w * other
        }
    }
}


impl <T: MulAssign> MulAssign<Vec4<T>> for Vec4<T>{
    fn mul_assign(&mut self, other:Vec4<T>){
            self.x *= other.x;
            self.y *= other.y;
            self.z *= other.z;
            self.w *= other.w;
    }
}

impl <T: MulAssign+Copy> MulAssign<T> for Vec4<T>{
    fn mul_assign(&mut self, other:T){
            self.x *= other;
            self.y *= other;
            self.z *= other;
            self.w *= other;

    }
}

impl <T: Div<Output=T> +  Copy> Div<T> for Vec4<T>{
    type Output = Vec4<T>;
    fn div(self, other:T) -> Vec4<T>{
        Vec4 {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
            w: self.w / other
        }
    }
}


impl <T: DivAssign + Copy> DivAssign<T> for Vec4<T>{
    fn div_assign(&mut self, other:T){
            self.x /= other;
            self.y /= other;
            self.z /= other;
            self.w /= other;
    }
}

impl<T: Neg<Output=T>> Neg for Vec4<T>{
    type Output = Vec4<T>;
    fn neg(self) -> Vec4<T>{
        Vec4 {
            x : -self.x,
            y : -self.y,
            z : -self.z,
            w : -self.w
        }
    }
}

impl<T: Add<Output=T> + Mul<Output=T> + Copy > Vec4<T>{
    fn dot(&self,other:&Vec4<T>) -> T {
        self.x * other.x  + self.y * other.y + self.z * other.z  + self.w * other.w
    }

    fn length2(&self) -> T{
        self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w
    }
}

impl<T> Index<usize> for Vec4<T>{
    type Output = T;
    fn index(&self, idx:usize) -> &T{
        match idx {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            3 => &self.w,
            _ => panic!("Out of Bounds")
        }
    }
} 


impl<T> IndexMut<usize> for Vec4<T>{
    fn index_mut<'a>(&'a mut self, idx:usize) -> &'a mut T{
        match idx {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            3 => &mut self.w,
            _ => panic!("Out of Bounds")
        }
    }
} 

type Vec4f = Vec4<f64>;


impl Vec4f {
    fn length(&self) -> f64 {
        self.length2().sqrt()
    }

    fn normalize(&mut self) {
        let len = self.length();
        if len > 0.0 {
            self.x /= len;
            self.y /= len;
            self.z /= len;
            self.w /= len;
        }

    }
}

/****************/



#[derive(Debug)]
struct Matrix33<T>{
    data: Vec3<Vec3<T>>
}


#[derive(Debug,Copy,Clone)]
struct Matrix44<T> {
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

impl<T:Mul<Output=T>+Add<Output=T>+Copy> Mul<Matrix44<T>> for Matrix44<T>{
    type Output = Matrix44<T>;
    fn mul(self,other:Matrix44<T>) -> Matrix44<T>{
        let other_t = other.transpose();
        let result = Matrix44::new(
            self[0].dot(&other_t[0]), self[0].dot(&other_t[1]), self[0].dot(&other_t[2]), self[0].dot(&other_t[3]),
            self[1].dot(&other_t[0]), self[1].dot(&other_t[1]), self[1].dot(&other_t[2]), self[1].dot(&other_t[3]),
            self[2].dot(&other_t[0]), self[2].dot(&other_t[1]), self[2].dot(&other_t[2]), self[2].dot(&other_t[3]),
            self[3].dot(&other_t[0]), self[3].dot(&other_t[1]), self[3].dot(&other_t[2]), self[3].dot(&other_t[3])
        );
        result
    }
}

impl<T:Copy> Matrix44<T>{
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
}

type Matrix44f = Matrix44<f64>;

impl Matrix44f{
    pub fn unit() -> Matrix44f{
        Matrix44f::new(
        1.0 ,0.0,0.0,0.0,
        0.0 ,1.0,0.0,0.0,
        0.0 ,0.0,1.0,0.0,
        0.0 ,0.0,0.0,1.0
        )
    }
    fn inverse(&self) -> Matrix44f{
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
fn main() {
    let mut x = Vec3 {
        x: 1, y:1 ,z:1
    };
    let y = Vec3f {
        x:1.0,y:1.0,z:1.0
    };
    let mx = Matrix44::new(1,1,1,1,
                            0,0,0,0,
                            0,0,0,0,
                            0,0,0,0
                            );
    
    let my = Matrix44::new(1,0,0,0,
                            1,0,0,0,
                            1,0,0,0,
                            1,0,0,0,
                            );

    let d = Matrix44f::new(0.707107, 0.0, -0.707107, 0.0, -0.331295, 0.883452, -0.331295, 0.0, 0.624695, 0.468521, 0.624695, 0.0, 4.000574, 3.00043, 4.000574, 1.0);

    let m = Matrix44::new(0.0,1.0,0.0,0.0,
                        1.0,0.0,0.0,0.0,
                        0.0,0.0,1.0,0.0,
                        0.0,0.0,0.0,1.0,
                        );
           
    println!("{:?}",d.inverse() * d);
}
