#include <chrono>
#include <iostream>

extern "C" int fib(int param);

int fib2(int n) {
  if (n <= 1) {
    return 1;
  }
  return fib2(n - 2) + fib2(n - 1);
}

int main(int argc, char** argv) {
  auto start = std::chrono::high_resolution_clock::now();
  std::cout << fib(30) << std::endl;
  auto end = std::chrono::high_resolution_clock::now();
  std::cout << (end - start).count() << std::endl;

  start = std::chrono::high_resolution_clock::now();
  std::cout << fib2(30) << std::endl;
  end = std::chrono::high_resolution_clock::now();
  std::cout << (end - start).count() << std::endl;
}