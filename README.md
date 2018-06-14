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
## Demo
在demo_c目录下有一个C语言版本的demo，是Freertos的7.1.0版本，可以直接在Windows环境下运行。编译程序时应当先编译所有的c文件，再将其链接为一个可执行文件（似乎dev C++是不行的，建议在windows下安装tdm-gcc）。输入指令如下：
>gcc -c main.c  
gcc -c tasks.c  
gcc -c queue.c  
gcc -c list.c  
gcc -c port.c  
gcc -c heap_3.c   
gcc -o main.exe main.o tasks.o queue.o list.o port.o heap_3.o  
main

程序可以运行。  
同样,在demo_rust目录下有一个Rust语言版本的demo（实际上应该说是Rust和C混合编程的），也可以在Windows环境下运行。安装好Rust环境后，在该目录下输入指令：
> cargo run

程序可以直接运行。
## 遇到的困难
在我们裁剪内核，试图运行一个demo时，发现了一个奇怪的问题：在C语言的情况下运行顺畅的程序，当我们把主程序换成rust语言时，程序可以编译通过但执行时会马上崩溃，报读写权限异常。  
最初，我们怀疑是在FFI交互时发生了错误，于是写了一个新的测试版本。rust的main函数直接调用了一个C函数，这个函数没有参数且没有返回值。按理说，不会有数据在两种语言间传递，也就不会发生这样的错误。但问题同样出现了，程序没有任何输出直接崩溃。  
而在我们换用C的main函数进行同样的调用时，程序可以正常运行——我们遭遇了玄学BUG。   
通过对比出错的demo和官方的demo。我们发现，我们自己编写的程序是64位的，而官方demo是32位的，问题可能出在这里。但是，我们自己用C写的测试程序也是64位的，却没有任何问题。  
经过艰难的调试，我们发现：在许多地方，程序采用强制类型转换获取数据。  
```C
pxThreadState = ( xThreadState *) *( ( unsigned long* ) pvOldCurrentTCB );
```
在读取内存时，将数据强制转换为32位的unsigned long类型，再转换为需要的类型。由于位数不够，地址的高32位被舍弃，只剩下了低32位，导致读写错误。因此，将unsigned long 改为unsigned long long 就可以解决问题。  
至于C的demo和rust的demo结果不同的问题，可能是因为两种编译器默认的内存地址不同。C分配的虚拟地址可能较小，高32位都被填充为0，因此地址截断对其没有影响。
