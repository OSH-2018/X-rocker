#include <stdio.h>
#include "FreeRTOS.h"
#include "task.h"
#include "queue.h"
#include "semphr.h"
static xSemaphoreHandle xMutexToDelete = NULL;
static char *pcStatusMessage = "OK";
void vAssertCalled( void )
{
	taskDISABLE_INTERRUPTS();
	for( ;; );
}
unsigned long *ptradd(unsigned long *p,unsigned short offset){
	return p+offset;
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
	//vTimerPeriodicISRTests();
}
void vApplicationIdleHook( void )
{
	
}
/*
void vApplicationIdleHook( void )
{
const unsigned long ulMSToSleep = 5;
xTaskHandle xIdleTaskHandle;
signed char *pcTaskName;
const unsigned char ucConstQueueNumber = 0xaaU, ucConstTaskNumber = 0x55U;


extern void vQueueSetQueueNumber( xQueueHandle pxQueue, unsigned char ucQueueNumber );
extern unsigned char ucQueueGetQueueNumber( xQueueHandle pxQueue );
extern unsigned char ucQueueGetQueueType( xQueueHandle pxQueue );
extern void vTaskSetTaskNumber( xTaskHandle xTask, unsigned portBASE_TYPE uxHandle );
extern unsigned portBASE_TYPE uxTaskGetTaskNumber( xTaskHandle xTask );


	Sleep( ulMSToSleep );


	xIdleTaskHandle = xTaskGetIdleTaskHandle();
	//xTimerTaskHandle = xTimerGetTimerDaemonTaskHandle();
	vTaskSetTaskNumber( xIdleTaskHandle, ( unsigned long ) ucConstTaskNumber );
	configASSERT( uxTaskGetTaskNumber( xIdleTaskHandle ) == ucConstTaskNumber );

	if( xTaskGetCurrentTaskHandle() != xIdleTaskHandle )
	{
		pcStatusMessage = "Error:  Returned idle task handle was incorrect";
	}


	if( xMutexToDelete != NULL )
	{
	
		vQueueSetQueueNumber( xMutexToDelete, ucConstQueueNumber );

		configASSERT( ucQueueGetQueueNumber( xMutexToDelete ) == ucConstQueueNumber );
		configASSERT( ucQueueGetQueueType( xMutexToDelete ) == queueQUEUE_TYPE_MUTEX );
		vSemaphoreDelete( xMutexToDelete );
		xMutexToDelete = NULL;
	}
}
-----------------------------------------------------------*/

