
#include <iostream>

extern "C" int add_numbers(uint32_t a, uint32_t b);
extern "C" int add_array(uint32_t a[], uint32_t length);

int main() {
    uint32_t result = add_numbers(1, 1);
    std::cout << "Hello, World! 1 + 1 allegedly equals " << result << std::endl;
    uint32_t array[] = {1, 2, 3, 4, 5};
    result = add_array(array, 5);
    std::cout << "The sum of 1, 2, 3, 4, 5 should equal 15 " << result << std::endl;

    return 0;
}
