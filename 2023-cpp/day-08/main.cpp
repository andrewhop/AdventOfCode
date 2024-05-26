#include <iostream>
#include <fstream>
#include <sstream>
#include <string>
#include <cstdint>
#include <vector>
#include <chrono>

uint16_t wordToNum(const char content[]) {
    return ((content[0] - 'A') << 10) + ((content[1] - 'A') << 5) + (content[2] - 'A');
}

std::string numToWord(uint16_t num) {
    char content[4];
    content[2] = 'A' + (num & 0x1F);
    content[1] = 'A' + ((num >> 5) & 0x1F);
    content[0] = 'A' + ((num >> 10) & 0x1F);
    content[3] = '\0';
    return std::string(content);
}

int day8p1(const std::basic_string<char>& content) {
    size_t index = 0;
    size_t instruction_end = 0;
    while(content[index] != '\n') {
        index++;
        instruction_end = index;
    }
    // skip newline and blank line
    index+=2;
    // build the map
    std::vector<uint16_t> map(2 * 26 * 26 * 26);
    while (index < content.length()) {
        uint16_t source = ((content[index++] - 'A') << 10) + ((content[index++] - 'A') << 5) + (content[index++] - 'A');
        index += 4;
        uint16_t left = ((content[index++] - 'A') << 10) + ((content[index++] - 'A') << 5) + (content[index++] - 'A');
        index+=2;
        uint16_t right = ((content[index++] - 'A') << 10) + ((content[index++] - 'A') << 5) + (content[index++] - 'A');
        index+=2;
        map[source * 2] = left * 2;
        map[source * 2 + 1] = right * 2;
    }

    // map done
    size_t count = 0;
    size_t position = 0;
    uint16_t end = wordToNum("ZZZ") * 2;
    while (position != end) {
        if (map[position] == 65535) {
            std::cout << "Invalid location: " << position << std::endl;
        }
        if (content[count % instruction_end] == 'L') {
            position = map[position];
        } else {
            position = map[position + 1];
        }
        count += 1;
    }

    return count;
}

std::basic_string<char> parse_file(std::string filePath) {
    std::ifstream file(filePath);
    if (!file.is_open()) {
        std::cerr << "Failed to open the file: " << filePath << std::endl;
        return "";
    }
    std::stringstream buffer;
    buffer << file.rdbuf();
    std::string fileContent = buffer.str();
    file.close();
    return fileContent;
}

int main(int argc, char* argv[]) {
    std::string filePath = ".";
    if (argc > 1) {
        filePath = argv[1];
    }

    const std::basic_string<char> &sampleContent = parse_file(filePath + "/sample.txt");
    int sampleResult = day8p1(sampleContent);
    std::cout << "Sample expected 6, got " << sampleResult << std::endl;

    const std::basic_string<char> &inputContent = parse_file(filePath + "/input.txt");
    int inputResult = day8p1(inputContent);
    std::cout << "Input expected 14429, got " << inputResult << std::endl;

    auto start = std::chrono::high_resolution_clock::now();
    uint64_t iterations = 10000;
    int total = 0;
    for (uint64_t i = 0; i < iterations; i++) {
        total += day8p1(inputContent);
    }
    auto end = std::chrono::high_resolution_clock::now();
    std::cout << "Unused total count to prevent optimization " << total << std::endl;
    double averageDuration = duration_cast<std::chrono::nanoseconds >(end - start).count()/ iterations;
    std::cout << "Average runtime over " << iterations << " iterations: " << averageDuration/1000 << " microseconds." << std::endl;

    return 0;
}
