extern crate libc;
use libc::{c_long,c_void,c_char,c_ushort,c_ulong};
use std::ffi::CString;
use std::ptr;
type xTaskHandle= *mut c_void;
static mut xTask2Handle: xTaskHandle=ptr::null_mut();
#[repr(C)]
pub struct xMemoryRegion
{
	pvBaseAddress: *mut c_void,
	ulLengthInBytes: c_ulong,
	ulParameters: c_ulong,
} 
#[link(name="other")]
extern "C"{
    fn vAssertCalled();
}
#[link(name="port")]
extern "C"{
    fn xPortStartScheduler();
}
#[link(name="heap_3")]
#[link(name="queue")]
#[link(name="list")]
#[link(name="tasks")]
extern "C"{
    static mut xIdleTaskHandle : xTaskHandle;
    static mut xSchedulerRunning : c_long;
    static mut xTickCount : c_ulong;
    fn vTaskDelay(xTicksToDelay: c_long);
    fn xTaskGenericCreate(pxTaskCode:extern fn(*mut c_void),pcName: *const c_char,usStackDepth: c_ushort,
                pvParameters:*mut c_void,uxPriority: c_long,pxCreatedTask:*mut (*mut c_void),
                puxStackBuffer: *mut c_long,xRegions: *const xMemoryRegion)->c_long;
    fn uxTaskPriorityGet(pxTask: xTaskHandle)->c_ulong;            
    fn vTaskPrioritySet(pxTask: xTaskHandle,uxNewPriority: c_ulong);   
}
extern fn vTask1(pvParameters: *mut c_void){ 
	let uxPriority:c_ulong; 
	/* 本任务将会比任务2更先运行，因为本任务创建在更高的优先级上。任务1和任务2都不会阻塞，所以两者要
	么处于就绪态，要么处于运行态。
	查询本任务当前运行的优先级 – 传递一个NULL值表示说“返回我自己的优先级”。 */ 
	unsafe{uxPriority = uxTaskPriorityGet( ptr::null_mut() );}
	loop
	{ 
		/* Print out the name of this task. */ 
		print!( "Task1 is running\r\n" ); 
		/* 把任务2的优先级设置到高于任务1的优先级，会使得任务2立即得到执行(因为任务2现在是所有任务
		中具有最高优先级的任务)。注意调用vTaskPrioritySet()时用到的任务2的句柄。程序清单24将展示
		如何得到这个句柄。 */ 
		print!( "About to raise the Task2 priority\r\n" ); 
		unsafe{vTaskPrioritySet( xTask2Handle, ( uxPriority +  1 ) );} 
		/* 本任务只会在其优先级高于任务2时才会得到执行。因此，当此任务运行到这里时，任务2必然已经执
		行过了，并且将其自身的优先级设置回比任务1更低的优先级。 */ 		
		unsafe{vTaskDelay(50)};	
	} 
} 
extern fn vTask2(pvParameters: *mut c_void){ 
	let uxPriority:c_ulong; 
	/* 本任务将会比任务2更先运行，因为本任务创建在更高的优先级上。任务1和任务2都不会阻塞，所以两者要
	么处于就绪态，要么处于运行态。
	查询本任务当前运行的优先级 – 传递一个NULL值表示说“返回我自己的优先级”。 */ 
	unsafe{uxPriority = uxTaskPriorityGet( ptr::null_mut() );}
	loop
	{ 
		/* 当任务运行到这里，任务1必然已经运行过了，并将本身务的优先级设置到高于任务1本身。 */ 
		print!( "Task2 is running\r\n" ); 
		/* 将自己的优先级设置回原来的值。传递NULL句柄值意味“改变我自己的优先级”。把优先级设置到低
		于任务1使得任务1立即得到执行 – 任务1抢占本任务。 */ 
		print!( "About to lower the Task2 priority\r\n" ); 
		unsafe{vTaskPrioritySet( ptr::null_mut(), ( uxPriority - 2 ) );}
		unsafe{vTaskDelay(50)};	
	} 
} 
extern fn prvIdleTask(pvParameters: *mut c_void){
}
fn vTaskStartScheduler(){
    let xReturn:c_long;
    unsafe{
        let name=CString::new("IDLE").unwrap();
        xReturn = xTaskGenericCreate(prvIdleTask,name.as_ptr(),50,ptr::null_mut(),
            0,&mut xIdleTaskHandle,ptr::null_mut(),ptr::null());    
    }          
    if xReturn==1 {
        /*portDISABLE_INTERRUPTS();
        //感觉这个函数是一个空函数，找不到它的实现，且调试时这条语句被直接跳过。
        */
        unsafe{
            xSchedulerRunning = 1;
		    xTickCount = 0;   
            /*portCONFIGURE_TIMER_FOR_RUN_TIME_STATS();
            //感觉这个函数是一个空函数，找不到它的实现，且调试时这条语句被直接跳过。
            */
            xPortStartScheduler(); //这个函数的分支块都是空的
        }     
    }
    else {
        unsafe{ vAssertCalled(); }
    }
}
fn main() {
    unsafe{
        let mut name=CString::new("Task1").unwrap();
        let p1:*mut c_void=ptr::null_mut();
        let p2:*mut(*mut c_void)=ptr::null_mut();
        xTaskGenericCreate(vTask1,name.as_ptr(),1000,p1,2,p2,ptr::null_mut(),ptr::null());
        name=CString::new("Task2").unwrap();
        xTaskGenericCreate(vTask2,name.as_ptr(),1000,p1,1,&mut xTask2Handle,ptr::null_mut(),ptr::null());
    }    
    vTaskStartScheduler();
    loop {
        
    }
}
