using i32 = int;
using b32 = i32;

struct Counter
{
    i32 current_count;
};

extern "C" void _count_up(Counter* counter);
extern "C" Counter* _create_counter(i32 initial);
extern "C" void _free_counter(Counter* counter);
extern "C" i32 _count(Counter* counter);
