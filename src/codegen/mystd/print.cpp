#include <cstdio>
#include <cmath>
#include <iomanip>
#include <sstream>
#include <time.h> 

#include "values.h"

extern "C" void print(Value* v) {
    if (v->type == DOUBLE) {
        if (std::fabs(x - std::round(x)) < 1e-9) {
            std::printf("%.0f\n", x);
        } else {
            std::stringstream stream;
            
            stream << std::fixed << std::setprecision(6) << x;
            std::string result = stream.str();

            result.erase(result.find_last_not_of('0') + 1, std::string::npos); 
            if (result.back() == '.') {
                result.pop_back();  
            }

            std::printf("%s\n", result.c_str());
        }
    } else if (v->type == BOOL) {
        std::printf("%s\n", v->data.b ? "true" : "false");
    }
}

extern "C" void print_time() {
    time_t my_time = time(NULL); 
  
    printf("%s", ctime(&my_time)); 
}

extern "C" Value* get_time() {
    time_t my_time = time(NULL); 
    return getDoubleValue(my_time);
}
