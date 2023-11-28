
#include <iostream>

extern "C" int add_numbers(int a, int b);

int main() {
    int result = add_numbers(100, 213);
    std::cout << "Hello, World! " << result << std::endl;
    return 0;
}
