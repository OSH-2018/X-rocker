extern crate libc;
use libc::{c_long,c_void,c_char,c_ushort,c_ulong};
use std::ffi::CString;
use std::ptr;
/*
#[repr(C)]
pub struct xMemoryRegion
{
	pvBaseAddress: *mut c_void,
	ulLengthInBytes: c_ulong,
	ulParameters: c_ulong,
} 
*/
//#[cfg(all(target_os = "win32", target_arch = "x86"))]
#[link(name="other")]
extern "C"{
    //fn vTask(pvParameters: *mut c_void);
    fn xStartTask();
}
#[link(name="port")]
#[link(name="heap_3")]
#[link(name="queue")]
#[link(name="list")]
#[link(name="tasks")]
extern "C"{
    /*
    fn vTaskDelay(xTicksToDelay:c_long);
    fn xTaskGenericCreate(pxTaskCode:extern fn(*mut c_void),pcName: *const c_char,usStackDepth: c_ushort,
                pvParameters:*mut c_void,uxPriority: c_long,pxCreatedTask:*mut (*mut c_void),
                puxStackBuffer: *mut c_long,xRegions: *const xMemoryRegion);*/
    fn vTaskStartScheduler();
}

/*
extern fn vTask(pvParameters: *mut c_void) {
    loop{
        print!("Task is running!\n");
        unsafe{vTaskDelay(50);}
    }
}
*/

fn main() {
    unsafe{
        //let name=CString::new("Task").unwrap();
        //let p1:*mut c_void=ptr::null_mut();
        //let p2:*mut(*mut c_void)=ptr::null_mut();
        xStartTask();
        //xTaskGenericCreate(vTask,name.as_ptr(),1000,p1,1,p2,ptr::null_mut(),ptr::null());
        print!("taskcreated");
        //vStartTimerDemoTask(50);
        //print!("timercreated");
        vTaskStartScheduler();
    }
    loop {
        
    }
}
