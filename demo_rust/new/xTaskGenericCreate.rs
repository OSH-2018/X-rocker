  extern crate libc;
use libc::{c_long,c_void,c_char,c_ushort,c_ulong};

fn  xTaskGenericCreate(pxTaskCode:extern fn(*mut libc::c_void),pcName:*const libc::c_char,usStackDepth:libc::c_ushort,pvParameters: *mut libc::c_void,
                       uxPriority:libc::c_long,pxCreatedTask:*mut(*mut libc::c_void),puxStackBuffer:*mut libc::c_long,xRegions:*const xMemoryRegion)->portBASE_TYPE
{
    let mut xReturn:portBASE_TYPE;
    let mut pxNewTCB:&mut tskTCB;
    configASSERT( pxTaskCode );
	configASSERT( ( uxPriority < configMAX_PRIORITIES ) );

    /* Allocate the memory required by the TCB and stack for the new task,
	checking that the allocation was successful. */
	pxNewTCB = prvAllocateTCBAndStack( usStackDepth, puxStackBuffer );

	if pxNewTCB != NULL
	{
		 let mut pxTopOfStack:&mut portSTACK_TYPE;

		if portUSING_MPU_WRAPPERS == 1
		{
			/* Should the task be created in privileged mode? */
            let mut xRunPrivileged:portBASE_TYPE;
			if ( uxPriority & portPRIVILEGE_BIT ) != 0 as libc::c_uint
			{
				xRunPrivileged = pdTRUE;
			}
			else
			{
				xRunPrivileged = pdFALSE;
			}
			uxPriority &= !portPRIVILEGE_BIT;
		}
		 /* portUSING_MPU_WRAPPERS == 1 */


        /* Calculate the top of stack address.  This depends on whether the
		stack grows from high memory to low (as per the 80x86) or visa versa.
		portSTACK_GROWTH is used to make the result positive or negative as
		required by the port. */
		if portSTACK_GROWTH < 0
		{
			pxTopOfStack = pxNewTCB.pxStack /*+ ( usStackDepth - 1 as libc::c_ushort )*/;
			pxTopOfStack = (pxTopOfStack as portPOINTER_SIZE_TYPE)&(!portBYTE_ALIGNMENT_MASK as portPOINTER_SIZE_TYPE ) as *mut portSTACK_TYPE ;

			/* Check the alignment of the calculated top of stack is correct. */
			configASSERT( ( ( pxTopOfStack as libc::u_long & portBYTE_ALIGNMENT_MASK as libc::u_long ) == 0 as libc::u_long ) );
		}
		else if portSTACK_GROWTH >= 0
		{
			pxTopOfStack = pxNewTCB.pxStack;

			/* Check the alignment of the stack buffer is correct. */
			configASSERT( ( (pxNewTCB.pxStack as libc::c_ulong &  portBYTE_ALIGNMENT_MASK as libc::c_ulong ) == 0 as libc::u_long ) );

			/* If we want to use stack checking on architectures that use
			a positive stack growth direction then we also need to store the
			other extreme of the stack space. */
			pxNewTCB.pxEndOfStack = pxNewTCB.pxStack + ( usStackDepth - 1 );
		}


		/* Setup the newly allocated TCB with the initial state of the task. */
		prvInitialiseTCBVariables( pxNewTCB, pcName, uxPriority, xRegions, usStackDepth );

		/* Initialize the TCB stack to look as if the task was already running,
		but had been interrupted by the scheduler.  The return address is set
		to the start of the task function. Once the stack has been initialised
		the	top of stack variable is updated. */
		if portUSING_MPU_WRAPPERS == 1
		{
			pxNewTCB.pxTopOfStack = pxPortInitialiseStack( pxTopOfStack, pxTaskCode, pvParameters, xRunPrivileged );
		}
		else if portUSING_MPU_WRAPPERS != 1
		{
			pxNewTCB.pxTopOfStack = pxPortInitialiseStack( pxTopOfStack, pxTaskCode, pvParameters );
		}

		/* Check the alignment of the initialised stack. */
		portALIGNMENT_ASSERT_pxCurrentTCB( ( ( pxNewTCB.pxTopOfStack as libc::c_ulong & portBYTE_ALIGNMENT_MASK as libc::c_long ) == 0 as libc::c_ulong ) );

		if pxCreatedTask as *mut libc::c_void != none
		{
			/* Pass the TCB out - in an anonymous way.  The calling function/
			task can use this as a handle to delete the task later if
			required.*/
			pxCreatedTask as *mut libc::c_void =  pxNewTCB as xTaskHandle;
		}

		/* We are going to manipulate the task queues to add this task to a
		ready list, so must make sure no interrupts occur. */
		taskENTER_CRITICAL();
		{
			uxCurrentNumberOfTasks = uxCurrentNumberOfTasks + 1 as portBASE_TYPE;
			if  pxCurrentTCB == none
			{
				/* There are no other tasks, or all the other tasks are in
				the suspended state - make this the current task. */
				pxCurrentTCB =  pxNewTCB;

				if uxCurrentNumberOfTasks ==  1 as portBASE_TYPE
				{
					/* This is the first task to be created so do the preliminary
					initialisation required.  We will not recover if this call
					fails, but we will report the failure. */
					prvInitialiseTaskLists();
				}
			}
			else
			{
				/* If the scheduler is not already running, make this task the
				current task if it is the highest priority task to be created
				so far. */
				if xSchedulerRunning == pdFALSE
				{
					if pxCurrentTCB.uxPriority <= uxPriority
					{
						pxCurrentTCB = pxNewTCB;
					}
				}
			}

			/* Remember the top priority to make context switching faster.  Use
			the priority in pxNewTCB as this has been capped to a valid value. */
			if pxNewTCB.uxPriority > uxTopUsedPriority
			{
				uxTopUsedPriority = pxNewTCB.uxPriority;
			}

			if configUSE_TRACE_FACILITY == 1
			{
				/* Add a counter into the TCB for tracing only. */
				pxNewTCB.uxTCBNumber = uxTCBNumber;
			}

			uxTCBNumber = uxTCBNumber + 1 as port_BASE_TPYE;

			prvAddTaskToReadyQueue( pxNewTCB );

			xReturn = pdPASS;
			traceTASK_CREATE( pxNewTCB );
		}
		taskEXIT_CRITICAL();
	}
	else
	{
		xReturn = errCOULD_NOT_ALLOCATE_REQUIRED_MEMORY;
		traceTASK_CREATE_FAILED();
	}

	if xReturn == pdPASS
	{
		if xSchedulerRunning != pdFALSE
		{
			/* If the created task is of a higher priority than the current task
			then it should run now. */
			if( pxCurrentTCB.uxPriority < uxPriority )
			{
				portYIELD_WITHIN_API();
			}
		}
	}

	xReturn
}

