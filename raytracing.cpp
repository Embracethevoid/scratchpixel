#include <cstdlib>
#include <cstdio>
#include <cmath>
#include <fstream>
#include <vector>
#include <istream>
#include <cassert>
#include <algorithm>

#if defined __linux__ || defined __APPLE__

#else
#define M_PI 3.141592653589793
#define INFINITY 1e8
#endif


template<typename T>
class Vec3{
    public: 
        T x,y,z;
        Vec3() : x(T(0)),y(T(0)),z(T(0)){}
        Vec3(T t):x(t),y(t),z(t){}
        Vec3(T xx,T yy,T zz):x(xx),y(yy),z(zz){}
        T length2() {
            return x*x + y*y+z*z;
        }
        T length(){
            return sqrt(length2());
        }
        Vec3 & normalize(){
            T nor2 = length2();
            if(nor2>0){
                T invNor = 1 / sqrt(nor2);
                x *= invNor;
                y*=invNor;
                z*=invNor;
            }
            return *this;
        }
        Vec3<T> operator * (const T &t) const {
            return Vec3<T>(x*t,y*t,z*t);
        }
        Vec3<T> operator * (const Vec3<T> &v) const {
            return Vec3<T>(x * v.x,y*v.x,z*v.x);
        }
        T dot (const Vec3<T> &v) const {
            return x *v.x + y * v.y + z * v.z;
        }
        Vec3<T> operator + (const Vec3<T> &v) const {
            return Vec3<T>(x + v.x,y+v.x,z+v.x);
        }
        Vec3<T> operator - (const Vec3<T> &v) const {
            return Vec3<T>(x - v.x,y-v.x,z-v.x);
        }

        Vec3<T> operator += (const Vec3<T> &v) {
            x += v.x;
            y += v.y;
            z += v.z;
            return *this;
        }

        Vec3<T> operator *= (const Vec3<T> &v)  {
            x *= v.x;
            y *= v.y;
            z *= v.z;
            return *this;
        }

        Vec3<T> operator - () const {
            return Vec3(-x,-y,-z);
        }

        friend std::ostream & operator << (std::ostream &os, const Vec3<T> &v){
            os << "[" <<v.x <<" " <<v.y <<" " <<v.z <<"]";
            return os;
        }

        
};


typedef Vec3<double> Vec3f;
typedef Vec3<double> Vec3d;
class Sphere{
    public:
        Vec3d center;
        double radius, radius2;
        Vec3d surfaceColor, emissionColor;
        double transparency, reflection;
        Sphere(const Vec3d &center,
               const double &radius,
               const Vec3d &surfaceColor,
               const double &reflection = 0,
               const double &transparency = 0,
               const Vec3d &emissionColor = 0):
               center(center),radius(radius),radius2(radius* radius),
               surfaceColor(surfaceColor),emissionColor(emissionColor),
               transparency(transparency),reflection(reflection)
               {}
               bool intersect(const Vec3d &rayOrigin, const Vec3d &rayDirection, double &t0, double &t1 ) const{
                   Vec3d l = center - rayOrigin;
                   double tca  = l.dot(rayDirection);
                   if(tca < 0) return false;
                   double d2 = l.dot(l) - tca* tca;
                   if(d2 > radius2) return false;
                   double thc = sqrt(radius2 - d2);
                   t0 = tca - thc;
                   t1 = tca + thc;
                   return true;
               }
};

#define MAX_RAY_DEPTH 5

double mix(const double & a ,const double & b,const double & mix){
    return b*mix + a* (1-mix);
}


Vec3d trace(
const Vec3d &rayOrigin,
const Vec3d &rayDirection,
const std::vector<Sphere> &spheres,
const int &depth) {
    double tnear = INFINITY;
    const Sphere* sphere = NULL;
    for(auto &s : spheres){
        double t0 = INFINITY, t1 = INFINITY;
        if(s.intersect(rayOrigin,rayDirection,t0,t1)){
            if(t0<0) t0 = t1;
            if(t0 < tnear){
                tnear = t0;
                sphere = &s;
            }
        }

    }
    if(!sphere) return Vec3d(2);
    Vec3d surfaceColor = 0;
    Vec3d pointOfIntersection = rayOrigin + rayDirection*tnear;
    Vec3d normalAtIntersection = pointOfIntersection - sphere->center;
    normalAtIntersection.normalize();

    double bias = 1e-4;
    bool inside = false;

    if(rayDirection.dot(normalAtIntersection) > 0 ) normalAtIntersection = - normalAtIntersection,inside = true;

    if((sphere->transparency > 0 || sphere->reflection > 0) && depth < MAX_RAY_DEPTH){
        double facingRatio = -rayDirection.dot(normalAtIntersection);
        double fresneleffect = mix(pow(1 - facingRatio,3),1,0.1);
        Vec3d reflectionDirection = rayDirection - normalAtIntersection * 2 * rayDirection.dot(normalAtIntersection);

        reflectionDirection.normalize();
        Vec3d reflection = trace(pointOfIntersection+normalAtIntersection * bias ,reflectionDirection,spheres,depth+1);
        Vec3d refraction = 0;
        if(sphere->transparency){
            double ior = 1.1, eta = inside ? ior : 1/ior;
            double cosi = -normalAtIntersection.dot(rayDirection);
            double k = 1 - eta * eta * (1 - cosi* cosi);
            Vec3d refractionDirection = rayDirection * eta + normalAtIntersection * (eta * cosi - sqrt(k));
            refractionDirection.normalize();
            refraction = trace(pointOfIntersection - normalAtIntersection*bias ,refractionDirection,spheres,depth+1);
        }
        surfaceColor = (reflection * fresneleffect + refraction * (1 - fresneleffect)*sphere->transparency) * sphere-> surfaceColor;


    } else {
        for(auto & s:spheres){
            if(s.emissionColor.x > 0){
                Vec3d transmission = 1;
                Vec3d lightDirection = s.center - pointOfIntersection;
                lightDirection.normalize();
                for(auto &s2 : spheres){
                    if(&s != &s2){
                        double t0,t1;
                        if(s2.intersect(pointOfIntersection + normalAtIntersection * bias,lightDirection,t0,t1)){
                            transmission = 0;
                            break;
                        }
                    }
                }
                surfaceColor += sphere->surfaceColor * transmission *
                std::max(double(0),normalAtIntersection.dot(lightDirection)) * s.emissionColor;
            }
        }
    }
    return surfaceColor + sphere -> emissionColor;
}

void render(const std::vector<Sphere> &spheres)
{
unsigned width = 640, height = 480;
Vec3f *image = new Vec3f[width * height], *pixel = image;
float invWidth = 1 / float(width), invHeight = 1 / float(height);
float fov = 30, aspectratio = width / float(height);
float angle = tan(M_PI * 0.5 * fov / 180.);
// Trace rays
for (unsigned y = 0; y < height; ++y) {
for (unsigned x = 0; x < width; ++x, ++pixel) {
float xx = (2 * ((x + 0.5) * invWidth) - 1) * angle * aspectratio;
float yy = (1 - 2 * ((y + 0.5) * invHeight)) * angle;
Vec3f raydir(xx, yy, -1);
raydir.normalize();
*pixel = trace(Vec3f(0), raydir, spheres, 0);
}
}
// Save result to a PPM image (keep these flags if you compile under Windows)
std::ofstream ofs("./untitled.ppm", std::ios::out | std::ios::binary);
ofs << "P6\n" << width << " " << height << "\n255\n";
for (unsigned i = 0; i < width * height; ++i) {
ofs << (unsigned char)(std::min(double(1), image[i].x) * 255) <<
(unsigned char)(std::min(double(1), image[i].y) * 255) <<
(unsigned char)(std::min(double(1), image[i].z) * 255);
}
ofs.close();
delete [] image;
} 

int main(int argc, char **argv)
{
srand48(13);
std::vector<Sphere> spheres;
// position, radius, surface color, reflectivity, transparency, emission color
spheres.push_back(Sphere(Vec3f( 0.0, -10004, -20), 10000, Vec3f(0.20, 0.20, 0.20), 0, 0.0));
spheres.push_back(Sphere(Vec3f( 0.0, 0, -20), 4, Vec3f(1.00, 0.32, 0.36), 1, 0.5));
spheres.push_back(Sphere(Vec3f( 5.0, -1, -15), 2, Vec3f(0.90, 0.76, 0.46), 1, 0.0));
spheres.push_back(Sphere(Vec3f( 5.0, 0, -25), 3, Vec3f(0.65, 0.77, 0.97), 1, 0.0));
spheres.push_back(Sphere(Vec3f(-5.5, 0, -15), 3, Vec3f(0.90, 0.90, 0.90), 1, 0.0));
// light
spheres.push_back(Sphere(Vec3f( 0.0, 20, -30), 3, Vec3f(0.00, 0.00, 0.00), 0, 0.0, Vec3f(3)));
render(spheres);

return 0;
} 
