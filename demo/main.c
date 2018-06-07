#include <stdio.h>
#include "FreeRTOS.h"
#include "task.h"
#include "queue.h"
#include "timers.h"
#include "semphr.h"
#define mainDELAY_LOOP_COUNT 50000
static const char *pcTextForTask1 = "Task 1 is running\r\n"; 

static const char *pcTextForTask2 = "Task 2 is running\r\n"; 
static xSemaphoreHandle xMutexToDelete = NULL;
static char *pcStatusMessage = "OK";
void vTaskFunction( void *pvParameters ) 
{ 
char *pcTaskName; 
volatile unsigned long ul; 
pcTaskName = ( char * ) pvParameters; 
/* As per most tasks, this task is implemented in an infinite loop. */ 
for( ;; ) 
{ 
/* Print out the name of this task. */ 
printf("%s",pcTaskName);
//vPrintString( pcTaskName ); 
/* Delay for a period. */ 
vTaskDelay(250);
} 
}
int main( void ) 
{ 
/* Create one of the two tasks. */ 
xTaskCreate( vTaskFunction, "Task 1", 1000, (void*)pcTextForTask1, 1, NULL ); 
xTaskCreate( vTaskFunction, "Task 2", 1000, (void*)pcTextForTask2, 1, NULL ); 
/* Start the scheduler so our tasks start executing. */ 
vTaskStartScheduler(); 
/* If all is well then main() will never reach here as the scheduler will 
now be running the tasks. If main() does reach here then it is likely that 
there was insufficient heap memory available for the idle task to be created. 
CHAPTER 5 provides more information on memory management. */ 
for( ;; ); 
} 

void vAssertCalled( void )
{
	taskDISABLE_INTERRUPTS();
	for( ;; );
}

void vApplicationMallocFailedHook( void )
{
	/* Can be implemented if required, but probably not required in this 
	environment and running this demo. */
}

void vApplicationTickHook( void )
{
	/* Call the periodic timer test, which tests the timer API functions that
	can be called from an ISR. */
	vTimerPeriodicISRTests();
}


void vApplicationIdleHook( void )
{
const unsigned long ulMSToSleep = 5;
xTaskHandle xIdleTaskHandle, xTimerTaskHandle;
signed char *pcTaskName;
const unsigned char ucConstQueueNumber = 0xaaU, ucConstTaskNumber = 0x55U;

/* These three functions are only meant for use by trace code, and not for
direct use from application code, hence their prototypes are not in queue.h. */
extern void vQueueSetQueueNumber( xQueueHandle pxQueue, unsigned char ucQueueNumber );
extern unsigned char ucQueueGetQueueNumber( xQueueHandle pxQueue );
extern unsigned char ucQueueGetQueueType( xQueueHandle pxQueue );
extern void vTaskSetTaskNumber( xTaskHandle xTask, unsigned portBASE_TYPE uxHandle );
extern unsigned portBASE_TYPE uxTaskGetTaskNumber( xTaskHandle xTask );

	/* Sleep to reduce CPU load, but don't sleep indefinitely in case there are
	tasks waiting to be terminated by the idle task. */
	Sleep( ulMSToSleep );

	/* Demonstrate the use of the xTimerGetTimerDaemonTaskHandle() and 
	xTaskGetIdleTaskHandle() functions.  Also try using the function that sets
	the task number. */
	xIdleTaskHandle = xTaskGetIdleTaskHandle();
	xTimerTaskHandle = xTimerGetTimerDaemonTaskHandle();
	vTaskSetTaskNumber( xIdleTaskHandle, ( unsigned long ) ucConstTaskNumber );
	configASSERT( uxTaskGetTaskNumber( xIdleTaskHandle ) == ucConstTaskNumber );

	/* This is the idle hook, so the current task handle should equal the 
	returned idle task handle. */
	if( xTaskGetCurrentTaskHandle() != xIdleTaskHandle )
	{
		pcStatusMessage = "Error:  Returned idle task handle was incorrect";
	}

	/* Check the timer task handle was returned correctly. */
	pcTaskName = pcTaskGetTaskName( xTimerTaskHandle );
	if( strcmp( pcTaskName, "Tmr Svc" ) != 0 )
	{
		pcStatusMessage = "Error:  Returned timer task handle was incorrect";
	}

	/* If xMutexToDelete has not already been deleted, then delete it now.
	This is done purely to demonstrate the use of, and test, the 
	vSemaphoreDelete() macro.  Care must be taken not to delete a semaphore
	that has tasks blocked on it. */
	if( xMutexToDelete != NULL )
	{
		/* Before deleting the semaphore, test the function used to set its
		number.  This would normally only be done from trace software, rather
		than application code. */
		vQueueSetQueueNumber( xMutexToDelete, ucConstQueueNumber );

		/* Before deleting the semaphore, test the functions used to get its
		type and number.  Again, these would normally only be done from trace
		software, rather than application code. */
		configASSERT( ucQueueGetQueueNumber( xMutexToDelete ) == ucConstQueueNumber );
		configASSERT( ucQueueGetQueueType( xMutexToDelete ) == queueQUEUE_TYPE_MUTEX );
		vSemaphoreDelete( xMutexToDelete );
		xMutexToDelete = NULL;
	}
}
/*-----------------------------------------------------------*/

