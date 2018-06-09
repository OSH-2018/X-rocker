# Rocker
hello-rust!  
team members:段逸凡，王浩宇，雷婷，陆万航，邱浩宸  
reference website: https://doc.rust-lang.org/std/  
In this website,we can search all source code of rust stdlib and explaintions,which benefits a lot.
## API
xTaskCreate(): It locates at xTaskGenericCreate() of tasks.c;  邱
vTaskStartScheduler(): It locates at vTaskStartScheduler() of tasks.c;  王
vTaskPrioritySet(): It locates at vTaskPrioritySet() of tasks.c;  邱
vTaskDelay(): It locates at vTaskDelay() of tasks.c;  陆
vTaskDelayUntil(): It locates at vTaskDelayUntil() of tasks.c;  陆
uxTaskPriorityGet(): It locates at uxTaskPriorityGet() of tasks.c;  雷
vTaskDelete(): It locates at vTaskDelete() of tasks.c;  雷
> 上述函数都是实现任务管理所必须的函数，可以在实现上述函数的基础上，仿照《FreeRTOS中文实用教程》中的示例，自行设计调度算法，实现一个Demo。
## 混合编程
参考网页： http://wiki.jikexueyuan.com/project/rust-primer/ffi/calling-ffi-function.html  
注意事项：
+ 需要下载libc和cc，需要有较好的网络环境（第一次使用时输入"Cargo run"自动下载）
+ 使用VS Code编程时，在终端页面输入"Cargo run"运行程序，不要使用右上角的Run Code按钮，否则所有的libc中的c类型都会报错
+ 作为库的c文件不要写main函数
+ 注意编写Cargo.toml和build.rs两个文件
+ 我写了一个示例程序，在仓库的example文件夹下
## Demo
在demo文件夹下有一个demo，是Freertos的7.1.0版本，可以直接在Windows环境下运行。编译程序时应当先编译所有的c文件，再将其链接为一个可执行文件（似乎dev C++是不行的，建议在windows下安装tdm-gcc）。
>gcc -c main.c  
gcc -c tasks.c  
......  
gcc -o main.exe main.o tasks.o ......  

附：
+ 内核应当进行裁剪。
