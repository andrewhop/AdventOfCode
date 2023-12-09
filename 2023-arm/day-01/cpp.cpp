#include <iostream>
#include <fstream>
#include <sstream>
#include <string>

extern "C" uint64_t puzzle1(const char* ptr, const char mapping[10][10]);


uint64_t puzzle1_helper(const char* filename) {
    const char mapping[10][10] = {{"zero"}, {"one"}, {"two"},
                                  {"three"}, {"four"}, {"five"},
                                  {"six"}, {"seven"}, {"eight"},
                                  {"nine"}};
    std::ifstream file(filename);
    std::stringstream buffer;
    buffer << file.rdbuf();
    file.close();
    std::string contents = buffer.str();
    const char* ptr = contents.c_str();
    uint64_t result = puzzle1(ptr, mapping);
    std::cout << filename << ": " << result << std::endl;
    return 0;
}


int main() {
    puzzle1_helper("sample1.txt");
    puzzle1_helper("puzzle1.txt");
}