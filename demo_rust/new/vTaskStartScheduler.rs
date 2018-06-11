extern crate libc;
use libc::{c_long,c_char,c_void,c_ushort,c_ulong};
use std::ffi::CString;
use std::ptr;
type xTaskHandle=*mut c_void;
#[repr(C)]
pub struct xMemoryRegion
{
	pvBaseAddress: *mut c_void,
	ulLengthInBytes: c_ulong,
	ulParameters: c_ulong,
} 
#[link(name="tasks")]
extern "C"{
    static mut xIdleTaskHandle : xTaskHandle;
    static mut xSchedulerRunning : c_long;
    static mut xTickCount : c_ulong;
    fn xTaskGenericCreate(pxTaskCode:extern fn(*mut c_void),pcName: *const c_char,usStackDepth: c_ushort,
                pvParameters:*mut c_void,uxPriority: c_long,pxCreatedTask:*mut xTaskHandle,
                puxStackBuffer: *mut c_long,xRegions: *const xMemoryRegion) -> c_long;
    fn prvIdleTask(pvParameters:*mut c_void);            
}
#[link(name="port")]
extern "C"{
    fn xPortStartScheduler();
}
#[link(name="other")]
extern "C"{
    fn vAssertCalled();
}
fn vTaskStartScheduler(){
    let xReturn:c_long;
    unsafe{
        let name=CString::new("IDLE").unwrap();
        xReturn = xTaskGenericCreate(prvIdleTask,name.as_ptr(),50,ptr::null_mut(),
            0,*mut xIdleTaskHandle,ptr::null_mut(),ptr::null());     
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