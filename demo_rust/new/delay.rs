extern crate libc;
use libc::{c_long,c_char,c_ulong,c_void};
type portTickType = c_ulong;
type portBASE_TYPE = c_long;

#[repr(C)]
pub struct xLIST_ITEM
{
    xItemValue: c_ulong,
	pvOwner: * mut c_ulong,
    pvContainer: * mut c_void,
} 

#[repr(C)]
pub struct tskTaskControlBlock
{
	pxTopOfStack : 							*mut c_ulong;		

	//xMPU_SETTINGS xMPUSettings;					
	
	xGenericListItem : 						 xLIST_ITEM;	
	xEventListItem : 						 xLIST_ITEM;		
	uxPriority:								 c_ulong;			
	pxStack:							 	 *mut c_ulong;			
	pcTaskName[ 12 ] :  					 c_char;

	pxEndOfStack : 							 *mut c_ulong;			
	uxCriticalNesting :     				 c_ulong;
	uxTCBNumber :           				 c_ulong;	
	uxTaskNumber :          				 c_ulong;	
	uxBasePriority :        				 c_ulong;
	pxTaskTag: 								 c_void;
	ulRunTimeCounter:       				 c_ulong;		

} tskTCB;

#[link(name = "other")]
extern "C"
{
    fn vAssertCalled();
}
#[link(name = "tasks")]
extern "C"
{
    static mut xTickCount : c_ulong;
    fn vTaskSuspendA11();
    fn xTaskResumeA11() -> c_ulong;
    fn prvAddCurrentTaskToDelayedList(xTimeToWake : c_ulong);
}
#[link(name = "list")]
extern "C"
{
    static mut pxCurrentTCB : tskTaskControlBlock;
    fn vListRemove (* xListItem);
}
#[link(name = "port")]
extern "C"{
    fn protYIED_WITHIN_API();
}
//all the functions in C needed

fn vTaskDelayUntil( pxPreviousWakeTime : *const c_ulong, xTimeIncrement : c_ulong)
{
    if  pxPreviousWaketime==0
    {
        unsafe{ vAssertCalled() };
    }
    if xTimeIncrement <= (0 as c_ulong)
    {
        unsafe{ vAssertCalled() };
    }
    //the two configASSERT
    unsafe{vTaskSuspendA11()};
    {
        if xTickCount < *pxPreviousWaketime
        {
            if  (xTimeToWake < pxPreviousWaketime.as_ptr()) && (xTimeToWake> xTickCount) 
                xShoudDelay = 1；
        }
        else
        {
            if (xTimeToWake < pxPreviousWakeTime.as_ptr()) || (xTimeToWake > xTickCount) 
				xShouldDelay = 1;
        }

        if xShouldDelay != 0
		{
			unsafe{traceTASK_DELAY_UNTIL()};
			unsafe{vListRemove( pxCurrentTCB->xGenericListItem.as_ptr() )};
			unsafe{prvAddCurrentTaskToDelayedList( xTimeToWake )};
		}
	}
	xAlreadyYielded = unsafe{xTaskResumeAll()};
    if xAlreadyYielded == 0 
	{
		unsafe{portYIELD_WITHIN_API()};
	}
}

fn vTaskDelay( xTicksToDelay: c_ulong)
{
	let mut xTimeToWake: c_ulong;
    let mut xAlreadyYielded: c_ulong = 0;
	if xTicksToDelay > (0 as c_ulong)
	{
		unsafe{vTaskSuspendAll()};
		{
			//traceTASK_DELAY();
			xTimeToWake = xTickCount + xTicksToDelay;
			unsafe{vListRemove( ( xListItem * ) &( pxCurrentTCB->xGenericListItem ) )};
			unsafe{prvAddCurrentTaskToDelayedList( xTimeToWake )};
		}
			xAlreadyYielded = unsafe{xTaskResumeAll()};
	}
	if xAlreadyYielded == 0 
	{
		unsafe{portYIELD_WITHIN_API()};
	}
}