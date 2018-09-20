#include <cstdlib>
#include <cstdio>
#include <iostream>
#include <iomanip>
#include <cmath>

template <typename T>
class Vec2
{
  public:
    T x, y;
    Vec2() : x(0), y(0) {}
    Vec2(T t) : x(t), y(t) {}
    Vec2(T xx, T yy) : x(xx), y(yy) {}
    Vec2 operator+(const Vec2 &v) { return Vec2(x + v.x, y + v.y); }
    Vec2 operator+=(const Vec2 &v)
    {
        x += v.x;
        y += v.y;
    }

    Vec2 operator-(const Vec2 &v) { return Vec2(x - v.x, y - v.y;) }
    Vec2 operator-=(const Vec2 &v)
    {
        x -= v.x;
        y -= v.y;
    }
    Vec2 operator*(const T t) { return Vec2(x * t, y * t); }
    Vec2 operator*=(const T t)
    {
        x *= t;
        y *= t;
    }
    Vec2 operator/(const T t) { return Vec2(x / t, y / t); }
    Vec2 operator/=(const T t)
    {
        x /= t;
        y /= t;
    }
    friend std::ostream &operator<<(std::ostream &s, const Vec2<T> &v)
    {
        return s << '[' << v.x << " , " << v.y << ']';
    }
    friend Vec2 operator*(const T r, const Vec2<T> &v)
    {
        return Vec2(r * v.x, r * v.y);
    }
};

typedef Vec2<float> Vec2f;
typedef Vec2<int> Vec2i;

template <typename T>
class Vec3
{
  public:
    T x, y, z;
    Vec3() : x(0), y(0), z(0) {}
    Vec3(T t) : x(t), y(t), z(t) {}
    Vec3(T xx, T yy, T zz) : x(xx), y(yy), z(zz) {}
    Vec3 operator+(const Vec3 &v) { return Vec3(x + v.x, y + v.y, z + v.z); }
    Vec3 operator+=(const Vec3 &v)
    {
        x += v.x;
        y += v.y;
        z += v.z;
    }

    Vec3 operator-(const Vec3 &v) { return Vec3(x - v.x, y - v.y, z - v.z); }
    Vec3 operator-=(const Vec3 &v)
    {
        x -= v.x;
        y -= v.y;
    }
    Vec3 operator*(const T t) { return Vec3(x * t, y * t, z * t); }
    Vec3 operator*=(const T t)
    {
        x *= t;
        y *= t;
        z *= t;
    }
    Vec3 operator/(const T t) { return Vec2(x / t, y / t, z / t); }
    Vec3 operator/=(const T t)
    {
        x /= t;
        y /= t;
        z /= t;
    }
    T dot(const Vec3 &v)
    {
        return x * v.x + y * v.y + z * v.z;
    }
    Vec3 crossProduct(Vec3 &v)
    {
        return Vec3(
            y * v.z - z * v.y,
            z * v.x - x * v.z,
            x * v.y - y * v.x);
    }
    T length2()
    {
        return x * x + y * y + z * z;
    }
    T length()
    {
        return sqrt(length2());
    }
    Vec3 normalize()
    {
        auto l = length();
        if (l > 0)
        {
            x /= l;
            y /= l;
            z /= l;
        }

        return *this;
    }
    friend std::ostream &operator<<(std::ostream &s, const Vec3<T> &v)
    {
        return s << '[' << v.x << " , " << v.y << " , " << v.z ']';
    }
    friend Vec3 operator*(const T r, const Vec3<T> &v)
    {
        return Vec3(r * v.x, r * v.y, r * v.z);
    }
};

typedef Vec3<float> Vec3f;
typedef Vec3<int> Vec3i;