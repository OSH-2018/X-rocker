# Rocker
hello-rust!  
team members:段逸凡，王浩宇，雷婷，陆万航，邱浩宸  
reference website: https://doc.rust-lang.org/std/  
In this website,we can search all source code of rust stdlib and explaintions,which benefits a lot.
## API
xTaskCreate(): It locates at xTaskGenericCreate() of tasks.c;  
vTaskStartScheduler(): It locates at vTaskStartScheduler() of tasks.c;  
vTaskPrioritySet(): It locates at vTaskPrioritySet() of tasks.c;  
vTaskDelay(): It locates at vTaskDelay() of tasks.c;  
vTaskDelayUntil(): It locates at vTaskDelayUntil() of tasks.c;  
uxTaskPriorityGet(): It locates at uxTaskPriorityGet() of tasks.c;  
vTaskDelete(): It locates at vTaskDelete() of tasks.c;  
> 上述函数都是实现任务管理所必须的函数，可以在实现上述函数的基础上，仿照《FreeRTOS中文实用教程》中的示例，自行设计调度算法，实现一个Demo。
