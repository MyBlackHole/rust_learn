#ifndef MATH_UTILS_H
#define MATH_UTILS_H

#ifdef __cplusplus
extern "C" {
#endif

// 简单的数学函数
int add(int a, int b);
double multiply(double a, double b);
void print_number(int num);

// 结构体示例
typedef struct {
  int x;
  int y;
} Point;

// 结构体操作函数
Point create_point(int x, int y);
double point_distance(Point p1, Point p2);

#ifdef __cplusplus
}
#endif

#endif
