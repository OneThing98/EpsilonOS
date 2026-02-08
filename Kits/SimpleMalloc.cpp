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
            void initialize(byte* base){
                m_base = base;
                printf("ChunkAllocator8 initialized at %p\n", base);
            }

        private:
            byte* m_base {nullptr};
    };
}
