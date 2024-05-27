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

size_t day8p1(const std::basic_string<char>& content) {
#if defined(PROFILE)
    auto start = std::chrono::high_resolution_clock::now();
#endif
    size_t instruction_end = content.find_first_of('\n');
    uint8_t* instructions = new uint8_t[instruction_end];
    for (size_t i = 0; i < instruction_end; i++) {
        instructions[i] = content[i] != 'L';
    }
    size_t index = instruction_end;

#if defined(PROFILE)
    auto end = std::chrono::high_resolution_clock::now();
    double nanosecondDuration = std::chrono::duration_cast<std::chrono::nanoseconds >(end - start).count();
    std::cout << "Finding end of instruction took: " << nanosecondDuration/1000 << " microseconds." << std::endl;
#endif
    // skip newline and blank line
    index+=2;

#if defined(PROFILE)
    start = std::chrono::high_resolution_clock::now();
#endif
    // build the map, max value is wordToNum("ZZZ") = 26425 * 2 = 52850
    uint16_t map[52850];
    size_t content_length = content.size();
    while (__builtin_expect((index < content_length), 1)) {
        uint16_t source = ((content[index] - 'A') << 10) + ((content[index+1] - 'A') << 5) + (content[index+2] - 'A');
        uint16_t left = ((content[index+7] - 'A') << 10) + ((content[index+8] - 'A') << 5) + (content[index+9] - 'A');
        uint16_t right = ((content[index+12] - 'A') << 10) + ((content[index+13] - 'A') << 5) + (content[index+14] - 'A');
        map[source * 2] = left * 2;
        map[source * 2 + 1] = right * 2;
        index+=17;
    }
#if defined(PROFILE)
    end = std::chrono::high_resolution_clock::now();
    nanosecondDuration = std::chrono::duration_cast<std::chrono::nanoseconds >(end - start).count();
    std::cout << "Building the map took: " << nanosecondDuration/1000 << " microseconds." << std::endl;
#endif

    // map done
#if defined(PROFILE)
    start = std::chrono::high_resolution_clock::now();
#endif

    size_t count = 0;
    size_t instruction_count = 0;
    size_t position = 0;
    // wordToNum("ZZZ") * 2;
    uint16_t target = 52850;
    while (__builtin_expect((position != target), 1)) {
        uint16_t offset = instructions[instruction_count];
        size_t next_position = position + offset;
        position = map[next_position];
        count++;
        instruction_count++;
        if (__builtin_expect((instruction_count == instruction_end), 0)) {
            instruction_count = 0;
        }
    }
#if defined(PROFILE)
    end = std::chrono::high_resolution_clock::now();
    nanosecondDuration = std::chrono::duration_cast<std::chrono::nanoseconds >(end - start).count();
    std::cout << "Solving problem took: " << nanosecondDuration/1000 << " microseconds." << std::endl << std::endl;
#endif
    delete[] instructions;

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
    uint64_t iterations = 10;
    int total = 0;
    for (uint64_t i = 0; i < iterations; i++) {
        total += day8p1(inputContent);
    }
    auto end = std::chrono::high_resolution_clock::now();
    std::cout << "Unused total count to prevent optimization " << total << std::endl;
    double nanosecondDuration = std::chrono::duration_cast<std::chrono::nanoseconds >(end - start).count()/ iterations;
    std::cout << "Average runtime over " << iterations << " iterations: " << nanosecondDuration/1000 << " microseconds." << std::endl;

    return 0;
}
