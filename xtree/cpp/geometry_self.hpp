#include <cstdib>
#include <cstdio>
#include <iostream>
#include <iomanip>
#include <cmath>


template <typename T>
class Vec2{
public:
    T x,y;
    Vec2():x(0),y(0){}
    Vec2(T t):x(t),y(t){}
    Vec2(T xx,T yy):x(xx),y(yy){}
    Vec2 operator + (const Vec2 &v){
        return Vec2(x+v.x, y + v.y);
    }
}