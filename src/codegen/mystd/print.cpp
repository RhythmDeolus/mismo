#include <cstdio>
#include <cmath>
#include <iomanip>
#include <sstream>
#include <time.h> 

extern "C" void print(double x) {
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
}

extern "C" void print_time() {
    time_t my_time = time(NULL); 
  
    printf("%s", ctime(&my_time)); 
}

extern "C" double get_time() {
    time_t my_time = time(NULL); 
    return static_cast<double>(my_time);
}
