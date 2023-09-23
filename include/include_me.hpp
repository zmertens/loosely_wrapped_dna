#pragma once

#include <memory>

// For rust::Str and other types
#include "rust/cxx.h"

namespace includeme {

class IncludeMe {
    
    public:

    IncludeMe();

    void me(rust::Str me) const;
};

std::unique_ptr<IncludeMe> new_include_me();

} // namespace includeme