#include "math_utils.h"
#include <math.h>
#include <stdio.h>

int add(int a, int b) { return a + b; }

double multiply(double a, double b) { return a * b; }

void print_number(int num) { printf("Number: %d\n", num); }

Point create_point(int x, int y) {
  Point p = {x, y};
  return p;
}

double point_distance(Point p1, Point p2) {
  int dx = p1.x - p2.x;
  int dy = p1.y - p2.y;
  return sqrt(dx * dx + dy * dy);
}
