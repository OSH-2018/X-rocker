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
## 运行
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
## Demo
在Demo中，我创建了3个任务。其中，任务1和任务3都是在主程序中创建的，任务2是由任务1创建的。通过这三个任务，我使用了前述的七个API。  
任务3在创建时的优先级最高，会最先运行。但它的功能是删除自身，所以只输出一句话，之后就不再出现。任务2由任务1创建，这两个任务的优先级交替提升,因此都有执行的机会，不会饿死。  
任务的执行流程如下：
![图片](https://github.com/OSH-2018/X-rocker/blob/master/pic/12.png?raw=true)  

```Rust
extern fn vTask1(_pvParameters: *mut c_void){ 
    let uxPriority:c_ulong; 
    let mut xTask2Handle: xTaskHandle=ptr::null_mut();
    /* 本任务将会比任务2更先运行，因为本任务创建在更高的优先级上。  
    任务1和任务2都不会阻塞，所以两者要么处于就绪态，要么处于运行态。  
    查询本任务当前运行的优先级 – 传递一个NULL值表示说“返回我自己的优先级”。*/ 
    uxPriority = uxTaskPriorityGet( ptr::null_mut() );  
    xTaskGenericCreate(vTask2,CString::new("Task2").unwrap().as_ptr(),1000,ptr::null_mut(),1,&mut xTask2Handle,ptr::null_mut(),ptr::null());
    /*任务2的创建*/
    loop{ 
        /* Print out the name of this task. */ 
        print!( "Task1 is running\r\n" ); 
        /* 把任务2的优先级设置到高于任务1的优先级，会使得任务2立即得到执行  
        (因为任务2现在是所有任务中具有最高优先级的任务)。  
        注意调用vTaskPrioritySet()时用到的任务2的句柄。*/ 
        print!( "About to raise the Task2 priority\r\n" ); 
        vTaskPrioritySet( xTask2Handle, uxPriority +  1  );
        /* 本任务只会在其优先级高于任务2时才会得到执行。  
        因此，当此任务运行到这里时，任务2必然已经执行过了，  
        并且将其自身的优先级设置回比任务1更低的优先级。 */ 		
        vTaskDelay(50);	
    } 
} 
extern fn vTask2(_pvParameters: *mut c_void){ 
    let uxPriority:c_ulong; 
    let mut xLastWakeTime:c_ulong;
    /* 本任务将会比任务2更先运行，因为本任务创建在更高的优先级上。  
    任务1和任务2都不会阻塞，所以两者要么处于就绪态，要么处于运行态。  
    查询本任务当前运行的优先级 – 传递一个NULL值表示说“返回我自己的优先级”。 */ 
    uxPriority = uxTaskPriorityGet( ptr::null_mut() );
    xLastWakeTime = xTaskGetTickCount();//获取当前时间
    loop{ 
        /* 当任务运行到这里，任务1必然已经运行过了，并将本身务的优先级设置到高于任务1本身。 */ 
        print!( "Task2 is running\r\n" ); 
        /* 将自己的优先级设置回原来的值。传递NULL句柄值意味“改变我自己的优先级”。  
        把优先级设置到低于任务1使得任务1立即得到执行 – 任务1抢占本任务。 */ 
        print!( "About to lower the Task2 priority\r\n" ); 
        vTaskPrioritySet( ptr::null_mut(), uxPriority - 2);
        vTaskDelayUntil(&mut xLastWakeTime, 50);	
    } 
} 
extern fn vTask3(_pvParameters: *mut c_void){
    loop{
        print!("Task3 is running.\r\n");
        print!("I'm going to delete myself.\r\n");
        vTaskDelete(ptr::null_mut());//删除当前任务
        print!("This sentence will never be printed.\r\n");
    }
}
fn main() {
    let name=CString::new("Task1").unwrap();
    xTaskGenericCreate(vTask1,name.as_ptr(),1000,ptr::null_mut(),2,ptr::null_mut(),ptr::null_mut(),ptr::null());
    //创建任务1
    xTaskGenericCreate(vTask3,CString::new("Task3").unwrap().as_ptr(),1000,ptr::null_mut(),1,ptr::null_mut(),ptr::null_mut(),ptr::null());
    //创建任务3
    vTaskStartScheduler();
    //开始任务执行
    loop {
        
    }
}
```

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
