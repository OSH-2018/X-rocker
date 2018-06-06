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
## 混合编程
参考网页： http://wiki.jikexueyuan.com/project/rust-primer/ffi/calling-ffi-function.html  
注意事项：
+ 需要下载libc和cc，需要有较好的网络环境（第一次使用时输入"Cargo run"自动下载）
+ 使用VS Code编程时，在终端页面输入"Cargo run"运行程序，不要使用右上角的Run Code按钮，否则所有的libc中的c类型都会报错
+ 作为库的c文件不要写main函数
+ 注意编写Cargo.toml和build.rs两个文件
+ 我写了一个示例程序，在仓库的example文件夹下
