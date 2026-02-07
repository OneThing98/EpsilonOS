// test_bitmap.cpp
#include "SimpleMalloc.cpp"
#include <cstdio>

int main() {
    byte data[10] = {0}; // 10 bytes = 80 bits
    SimpleMalloc::AllocationBitmap bitmap(data, 80);
    
    bitmap.set(0, true);
    bitmap.set(5, true);
    bitmap.set(17, true);
    
    printf("Bit 0: %d (should be 1)\n", bitmap.get(0));
    printf("Bit 1: %d (should be 0)\n", bitmap.get(1));
    printf("Bit 5: %d (should be 1)\n", bitmap.get(5));
    printf("Bit 17: %d (should be 1)\n", bitmap.get(17));
    
   
    bitmap.set(5, false);
    printf("Bit 5 after unset: %d (should be 0)\n", bitmap.get(5));
    
    printf("Bitmap test passed!\n");
    return 0;
}
