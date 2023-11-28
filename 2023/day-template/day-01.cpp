
#include <iostream>

extern "C" int add_numbers(int a, int b);

int main() {
    int result = add_numbers(1, 1);
    std::cout << "Hello, World! 1 + 1 allegedly equals " << result << std::endl;
    return 0;
}
