// https://www.youtube.com/watch?v=wQxj20X-tIU
// this ties into the first part of effective modern cpp

// Notes
// c++98 type deduction is used only for templated
// c++11 scope expands, auto, universal ref, lambda captures, ...
// c++14 expanded even furthur

// general problem: need to deduce T and Paramtype from expr
// T is often different from Paramtype
// template <typename T>
// void f(Paramtype param)
// f(expr)

// three general casses
// 1) Paramtype is a reference or pointer, but not a universal reference
// - if expr's type is a reference, ignore that
// - pattern match exprs type against ParamType to determine T
// - auto plays the same role as T
// 2) Paramtype is a universal reference
// template<typename T>
// void f(T&& param);
// f(expr);
// - treated like normal ref papram expect if expr is lvalue with deduced type E, T is deduced as E&
// - only place in type deduction where T will be a refernce
// 3) Paramtype is neither a reference nor a pointer, by value
// - if exprs type is a reference, ignore that
// - is expr is const or volatile, ignore that
// - T is the result

// when using auto, it plays the role of T, ref/const is dropped
// auto is never deduced to be a reference, it must be manually added
// creates a brand new object unless it is adorned with references/pointers
// this is for c++ 14
// difference when you have braced initializer 
// - a braced initializer has no type
// - this will fail type deduction
// - BUT, if you use auto a type will be deduced... an initilizer list
// after N3922, c++17
// using auto but dont use equal with initializer list
// - if one element, type of auto is the type of element inside
//

