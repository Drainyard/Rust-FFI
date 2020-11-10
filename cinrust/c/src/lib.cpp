#include <stdlib.h>
#include "lib.h"

extern "C" void _count_up(Counter* counter)
{
    if(counter)
    {
        counter->current_count++;
    }
}

extern "C" Counter* _create_counter(i32 initial)
{
    Counter* counter = (Counter*)malloc(sizeof(Counter));
    counter->current_count = initial;
    return counter;
}

extern "C" void _free_counter(Counter* counter)
{
    if(counter)
    {
        free(counter);
    }
}

extern "C" i32 _count(Counter* counter)
{
    if(counter)
    {
        return counter->current_count;
    }
    return -1;
}
