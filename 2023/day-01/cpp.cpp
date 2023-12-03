#include <iostream>
#include <fstream>
#include <sstream>
#include <string>

extern "C" uint64_t puzzle1(const char* ptr);

uint64_t puzzle1_helper(const char* filename) {
    std::ifstream file(filename);
    std::stringstream buffer;
    buffer << file.rdbuf();
    file.close();
    std::string contents = buffer.str();
    const char* ptr = contents.c_str();
    uint64_t result = puzzle1(ptr);
    std::cout << filename << ": " << result << std::endl;
    return 0;
}


int main() {
    puzzle1_helper("sample1.txt");
    puzzle1_helper("puzzle1.txt");
}