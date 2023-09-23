#include "loosely_wrapped_dna/include/include_me.hpp"
#include "loosely_wrapped_dna/src/main.rs.h"

#include <iostream>
#include <string_view>
#include <string>

#include <boost/algorithm/algorithm.hpp>
#include <boost/algorithm/string/case_conv.hpp>

namespace includeme {

IncludeMe::IncludeMe() {
    using namespace std;

    cout << "IncludeMe()" << endl;
}

void IncludeMe::me(rust::Str me) const {
    using namespace std;

    typedef boost::iterator_range<std::string::iterator> range;

    string_view v (me.data());

    cout << v << endl;

    std::string me2 ("this is me2");
    range r (me2.begin(), me2.end());

    boost::to_upper(r);

    cout << r << endl;
}

std::unique_ptr<IncludeMe> new_include_me() {
    return std::make_unique<IncludeMe>();
}

} // namespace includeme