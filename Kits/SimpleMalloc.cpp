#include "SimpleMalloc.h"
#include<cstdio>
#include "Assertions.h"
#include "Types.h"

namespace SimpleMalloc {

    class AllocationBitmap {
        public:
            static AllocationBitmap wrap(byte* data, unsigned size){
                return AllocationBitmap(data,size);
            }
            bool get(unsigned index) const {
                ASSERT(index < m_size);
                // index/8 -> which byte?
                // index%8 -> which bit inside that byte
                // the & operation checks if the bit is on/off
                // for ex. 01001000   (actual data) & 00001000   (mask)
                //   00001000 <- non-zero ->bit is ON
                return 0 != (m_data[index/8] & (1u << (index % 8)));
            }

            void set(unsigned index, bool value) const {
                ASSERT(index < m_size);
                if(value)
                    m_data[index/8] |= static_cast<byte>((1u<<(index%8)));
                else
                    m_data[index/8] &= static_cast<byte>(~(1u <<(index % 8)));
            }

        private:
            AllocationBitmap(byte* data, unsigned size)
                : m_data(data)
                , m_size(size)
                {

                }
             byte* m_data{nullptr};
             unsigned m_size {0};
    };
    
    class ChunkAllocator8{
        public:
            static constexpr dword CHUNK_SIZE=8;

            static constexpr dword capacityInAllocations() {
                return 1048576 / CHUNK_SIZE; //1MB /8 bytes = 131,072 chunks

            }

            static constexpr dword capacityInBytes() {
                return capacityInAllocations() * CHUNK_SIZE;
            }

            static constexpr dword sizeOfAllocationBitmapInBytes()
            {
                return capacityInAllocations() / 8;
            }

            void initialize(byte* base){
                m_base = base;
                printf("ChunkAllocator8:\n");
                printf(" Can hold: %u chunks\n", capacityInAllocations());
                printf(" Bitmap size: %u bytes \n", sizeOfAllocationBitmapInBytes());
                printf("Data size: %u bytes\n", capacityInBytes());
            }

        private:
            byte* m_base {nullptr};
    };
}
