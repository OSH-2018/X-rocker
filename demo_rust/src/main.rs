extern crate libc;
use libc::{c_long,c_void,c_char,c_ushort,c_ulong};
use std::ffi::CString;
use std::ptr;
use std::mem;
type xTaskHandle= *mut c_void;
type portBASE_TYPE= c_long;
type portSTACK_TYPE= c_ulong;
static mut xTask2Handle: xTaskHandle=ptr::null_mut();
#[repr(C)]
pub struct xMemoryRegion{
	pvBaseAddress: *mut c_void,
	ulLengthInBytes: c_ulong,
	ulParameters: c_ulong,
} 
#[repr(C)]
pub struct xListItem{
    xItemValue: c_ulong,
    pxNext: *mut xListItem,
    pxPrevious: *mut xListItem,
    pvOwner: *mut c_void,
    pvContainer: *mut c_void, 
}
#[repr(C)]
pub struct xList{
    uxNumberOfItems: c_ulong,
	pxIndex: *mut xListItem,		
	xListEnd: *mut xMiniListItem,
}
#[repr(C)]
pub struct xMiniListItem{
    xItemValue: c_ulong,
	pxNext: *mut xListItem,
	pxPrevious: *mut xListItem,
}
#[repr(C)]
pub struct tskTCB{
    pxTopOfStack: *mut portSTACK_TYPE,
    xGenericListItem: xListItem,
    xEventListItem: xListItem,
    uxPriority: c_ulong,
    pxStack: *mut portSTACK_TYPE,
    pcTaskName: [c_char;12],
    uxTCBNumber: c_ulong,
    uxTaskNumber: c_ulong,
    uxBasePriority: c_ulong,
}
impl tskTCB {
    fn new()->tskTCB{
        //let a=vec![0 as c_long;usStackDepth as usize];
        //let point=Box::new(a);
        tskTCB{
            pxTopOfStack: ptr::null_mut(),
            xGenericListItem: unsafe{mem::uninitialized()},
            xEventListItem: unsafe{mem::uninitialized()},
            uxPriority: 0,
            pxStack: ptr::null_mut(),
            pcTaskName: unsafe{mem::uninitialized()},
            uxTCBNumber: 0,
            uxTaskNumber: 0,
            uxBasePriority: 0,
        }
    }
}
#[link(name="other")]
extern "C"{
    fn vAssertCalled();
    fn ptradd(p:*mut c_ulong,offset:c_ushort)->*mut c_ulong;
}
#[link(name="port")]
extern "C"{
    fn xPortStartScheduler();
    fn vPortEnterCritical();
    fn vPortExitCritical(); 
    fn vPortGenerateSimulatedInterrupt(ulInterruptNumber: c_ulong);
}
#[link(name="heap_3")]
#[link(name="queue")]
#[link(name="list")]
extern "C"{
    fn vListRemove (pxItemToRemove: *mut xListItem);
    fn vListInsertEnd(pxList: *mut xList,pxNewListItem: *mut xListItem);
}
#[link(name="tasks")]
extern "C"{
    static mut xIdleTaskHandle : xTaskHandle;
    static mut xSchedulerRunning : c_long;
    static mut xTickCount : c_ulong;
    static mut pxCurrentTCB : *mut tskTCB;
    static mut uxCurrentNumberOfTasks : c_ulong; 	
    static mut uxTopUsedPriority: c_ulong;
    static mut uxTCBNumber : c_ulong;
    static mut uxTopReadyPriority : c_ulong;
    static mut pxReadyTasksLists : [xList;7];
    fn xTaskGenericCreate(pxTaskCode:extern fn(*mut c_void),pcName: *const c_char,usStackDepth: c_ushort,
                pvParameters:*mut c_void,uxPriority: c_long,pxCreatedTask:*mut (*mut c_void),
                puxStackBuffer: *mut c_long,xRegions: *const xMemoryRegion)->c_long;         
    fn vTaskPrioritySet(pxTask: xTaskHandle,uxNewPriority: c_ulong);  
    fn vTaskSuspendAll();
    fn xTaskResumeAll()->c_long;
    fn prvACTTDL(xTimeToWake: c_ulong);
    fn prvAllocateTCBAndStack(usStackDepth:c_ushort,puxStackBuffer:*mut portSTACK_TYPE)->*mut tskTCB;
    fn prvInitialiseTCBVariables(pxTCB: *mut tskTCB,pcName: *const c_char,uxPriority:c_ulong,
                xRegions: *const xMemoryRegion,usStackDepth: c_ushort);
    fn pxPortInitialiseStack(pxTopOfStack:*mut c_ulong,pxCode:extern fn(*mut c_void),
                pvParameters:*mut c_void)->*mut c_ulong; 
    fn prvInitialiseTaskLists();                  
}
extern fn vTask1(pvParameters: *mut c_void){ 
	let uxPriority:c_ulong; 
	/* 本任务将会比任务2更先运行，因为本任务创建在更高的优先级上。任务1和任务2都不会阻塞，所以两者要
	么处于就绪态，要么处于运行态。
	查询本任务当前运行的优先级 – 传递一个NULL值表示说“返回我自己的优先级”。 */ 
	uxPriority = uxTaskPriorityGet( ptr::null_mut() );
	loop
	{ 
		/* Print out the name of this task. */ 
		print!( "Task1 is running\r\n" ); 
		/* 把任务2的优先级设置到高于任务1的优先级，会使得任务2立即得到执行(因为任务2现在是所有任务
		中具有最高优先级的任务)。注意调用vTaskPrioritySet()时用到的任务2的句柄。程序清单24将展示
		如何得到这个句柄。 */ 
		print!( "About to raise the Task2 priority\r\n" ); 
		unsafe{vTaskPrioritySet( xTask2Handle, uxPriority +  1  );} 
		/* 本任务只会在其优先级高于任务2时才会得到执行。因此，当此任务运行到这里时，任务2必然已经执
		行过了，并且将其自身的优先级设置回比任务1更低的优先级。 */ 		
		vTaskDelay(50);	
	} 
} 
extern fn vTask2(pvParameters: *mut c_void){ 
	let uxPriority:c_ulong; 
    let mut xLastWakeTime:c_ulong;
	/* 本任务将会比任务2更先运行，因为本任务创建在更高的优先级上。任务1和任务2都不会阻塞，所以两者要
	么处于就绪态，要么处于运行态。
	查询本任务当前运行的优先级 – 传递一个NULL值表示说“返回我自己的优先级”。 */ 
	uxPriority = uxTaskPriorityGet( ptr::null_mut() );
    unsafe{xLastWakeTime = xTaskGetTickCount();}
	loop
	{ 
		/* 当任务运行到这里，任务1必然已经运行过了，并将本身务的优先级设置到高于任务1本身。 */ 
		print!( "Task2 is running\r\n" ); 
		/* 将自己的优先级设置回原来的值。传递NULL句柄值意味“改变我自己的优先级”。把优先级设置到低
		于任务1使得任务1立即得到执行 – 任务1抢占本任务。 */ 
		print!( "About to lower the Task2 priority\r\n" ); 
		unsafe{vTaskPrioritySet( ptr::null_mut(), uxPriority - 2);}
        vTaskDelayUntil(&mut xLastWakeTime, 50);
		//vTaskDelay(50);	
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
fn uxTaskPriorityGet(pxTask:xTaskHandle)->c_ulong{
	let pxTCB:*mut tskTCB ;
	let uxReturn:c_ulong;
	unsafe{vPortEnterCritical();}
	{
		/* If null is passed in here then we are changing the
		priority of the calling function. */
        if pxTask==ptr::null_mut(){
            unsafe{pxTCB=pxCurrentTCB;}
        }
        else{
            pxTCB=pxTask as *mut tskTCB;
        } 
    	unsafe{uxReturn = (*pxTCB).uxPriority};
	}
	unsafe{vPortExitCritical();}
	uxReturn
}
fn vTaskDelay( xTicksToDelay: c_ulong){
	let xTimeToWake: c_ulong;
    let mut xAlreadyYielded: c_long = 0;
	if xTicksToDelay > (0 as c_ulong){
		unsafe{vTaskSuspendAll()};
		{
			unsafe{
                xTimeToWake = xTickCount + xTicksToDelay;
                vListRemove(&mut (*pxCurrentTCB).xGenericListItem);
                prvACTTDL( xTimeToWake );
            }
		}
		xAlreadyYielded = unsafe{xTaskResumeAll()};
	}
	if xAlreadyYielded == 0 {
		unsafe{vPortGenerateSimulatedInterrupt(0)};
	}
}
fn vTaskDelayUntil( pxPreviousWakeTime : *mut c_ulong, xTimeIncrement : c_ulong){
    let mut xShouldDelay: c_long=0;
    let xTimeToWake: c_ulong;
    let xAlreadyYielded: c_long;
    if  pxPreviousWakeTime==ptr::null_mut()
    {
        unsafe{ vAssertCalled() };
    }
    if xTimeIncrement <= (0 as c_ulong)
    {
        unsafe{ vAssertCalled() };
    }
    //the two configASSERT
    unsafe{vTaskSuspendAll()};
    unsafe{
        xTimeToWake = *pxPreviousWakeTime + xTimeIncrement;
        if xTickCount < *pxPreviousWakeTime
        {
            if  (xTimeToWake < *pxPreviousWakeTime) && (xTimeToWake> xTickCount){
                xShouldDelay = 1;
            } 
                
        }
        else
        {
            if (xTimeToWake < *pxPreviousWakeTime) || (xTimeToWake > xTickCount){
                xShouldDelay = 1;
            } 				
        }
        *pxPreviousWakeTime = xTimeToWake;
    }
    if xShouldDelay != 0
		{
			//unsafe{traceTASK_DELAY_UNTIL()};//这是一个空函数
			unsafe{vListRemove(&mut (*pxCurrentTCB).xGenericListItem)};
			unsafe{prvACTTDL( xTimeToWake)};
		}
	xAlreadyYielded = unsafe{xTaskResumeAll()};
    if xAlreadyYielded == 0 
	{
		unsafe{vPortGenerateSimulatedInterrupt(0)};
	}
}
fn xTaskGetTickCount()->c_ulong {
    let xTicks:c_ulong;
    unsafe{
        vPortEnterCritical();
        xTicks=xTickCount;
        vPortExitCritical();
    }
    xTicks
}
/*
fn xTaskGenericCreate(pxTaskCode:extern fn(*mut c_void),pcName: *const c_char,usStackDepth: c_ushort,
                pvParameters:*mut c_void,uxPriority: c_ulong,pxCreatedTask:*mut (*mut c_void),
                puxStackBuffer: *mut c_ulong,xRegions: *const xMemoryRegion)->c_long{
    let mut xReturn: c_long=0;
    let mut pxNewTCB: *mut tskTCB;
    if uxPriority>=7{
        unsafe{vAssertCalled();}
    }
    pxNewTCB=&mut tskTCB::new();
    let mut a=vec![0 as c_ulong;usStackDepth as usize];
    print!("0");
    unsafe{
        (*pxNewTCB).pxStack=&mut a[0];
        //(*pxNewTCB).pxTopOfStack=&mut a[(usStackDepth-1)as usize];
        //pxNewTCB=prvAllocateTCBAndStack( usStackDepth, puxStackBuffer );
    }
    print!("1");
    if pxNewTCB!=ptr::null_mut(){
        let mut pxTopOfStack:*mut portSTACK_TYPE;
        unsafe{
            //pxTopOfStack=ptradd((*pxNewTCB).pxStack,usStackDepth-1);
            pxTopOfStack=&mut a[(usStackDepth-1)as usize];
            prvInitialiseTCBVariables( pxNewTCB, pcName, uxPriority, xRegions, usStackDepth );
            (*pxNewTCB).pxTopOfStack = pxPortInitialiseStack( pxTopOfStack, pxTaskCode, pvParameters );
        }
        print!("2");
        if pxCreatedTask!=ptr::null_mut(){
            unsafe{*pxCreatedTask =pxNewTCB as xTaskHandle;}
        }
        unsafe{
            vPortEnterCritical();
            uxCurrentNumberOfTasks+=1; 	
            if pxCurrentTCB==ptr::null_mut(){
                pxCurrentTCB=pxNewTCB;
                if uxCurrentNumberOfTasks==1 {
					prvInitialiseTaskLists();
				}
            }
            else{
                if xSchedulerRunning == 0 {
					if (*pxCurrentTCB).uxPriority <= uxPriority {
						pxCurrentTCB = pxNewTCB;
					}
				}
            }
            print!("3");
            if (*pxNewTCB).uxPriority > uxTopUsedPriority {
				uxTopUsedPriority = (*pxNewTCB).uxPriority;
			}
            (*pxNewTCB).uxTCBNumber = uxTCBNumber;
            uxTCBNumber+=1;
            if (*pxNewTCB).uxPriority > uxTopReadyPriority {																													
		        uxTopReadyPriority = (*pxNewTCB).uxPriority;																		
	        }																													
	        vListInsertEnd( &mut ( pxReadyTasksLists[ (*pxNewTCB ).uxPriority as usize ] ), &mut ( (*pxNewTCB).xGenericListItem ) );
            xReturn=1;
            vPortExitCritical();
            if xSchedulerRunning != 0 {
			    if (*pxCurrentTCB).uxPriority < uxPriority {
				    vPortGenerateSimulatedInterrupt(0);
		    	}
		    }
        }
    }
    else{
        xReturn=-1;
    }
    xReturn
}
*/
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
//other.c中的函数可以改写到main.rs中