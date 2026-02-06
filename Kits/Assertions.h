#pragma once

#include <assert.h>

#define ASSERT(x) assert(x)
#define ASSERT_NOT_REACHED() assert(false)

namespace Kits {
    inline void notImplemented() {
        assert(false);
    }
}

using Kits::notImplemented;
