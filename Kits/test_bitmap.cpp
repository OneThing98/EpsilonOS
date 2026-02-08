// test_bitmap.cpp
#include "SimpleMalloc.cpp"
#include <cstdio>

int main() {
    byte memory[1000 + 16384];
    SimpleMalloc::ChunkAllocator8 allocator;
    allocator.initialize(memory);
    return 0;

}
