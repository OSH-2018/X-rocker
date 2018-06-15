#include <stdio.h>
#include "FreeRTOS.h"
#include "task.h"
#include "queue.h"
#include "semphr.h"
#define mainTIMER_TEST_PERIOD			( 250 )
static xSemaphoreHandle xMutexToDelete = NULL;
static char *pcStatusMessage = "OK";
void vTask3( void *pvParameters)
{
	for (;;)
	{
		printf("Task3 is running.\r\n");
		printf("I'm going to delete myself.\r\n");
		vTaskDelete(NULL);
		printf("This sentence will never be printed.\r\n");
	}
}
void vTask2( void *pvParameters ) 
{ 
	unsigned portBASE_TYPE uxPriority; 
	/* 任务1比本任务更先启动，因为任务1创建在更高的优先级。任务1和任务2都不会阻塞，所以两者要么处于
	就绪态，要么处于运行态。
	查询本任务当前运行的优先级 – 传递一个NULL值表示说“返回我自己的优先级”。 */ 
	uxPriority = uxTaskPriorityGet( NULL ); 
	xTaskCreate( vTask3, "Task 3", 1000, NULL, 3, NULL );//创建任务3
	for( ;; ) 
	{ 
		/* 当任务运行到这里，任务1必然已经运行过了，并将本身务的优先级设置到高于任务1本身。 */ 
		printf( "Task2 is running\r\n" ); 
		/* 将自己的优先级设置回原来的值。传递NULL句柄值意味“改变我自己的优先级”。把优先级设置到低
		于任务1使得任务1立即得到执行 – 任务1抢占本任务。 */ 
		printf( "About to lower the Task2 priority\r\n" ); 
		vTaskPrioritySet( NULL, ( uxPriority - 2 ) );
		vTaskDelay(50);
	}
}
void vTask1( void *pvParameters ) 
{ 
	unsigned portBASE_TYPE uxPriority; 
	xTaskHandle xTask2Handle; //任务2的句柄
	/* 本任务将会比任务2更先运行，因为本任务创建在更高的优先级上。任务1和任务2都不会阻塞，所以两者要
	么处于就绪态，要么处于运行态。
	查询本任务当前运行的优先级 – 传递一个NULL值表示说“返回我自己的优先级”。 */ 
	uxPriority = uxTaskPriorityGet( NULL ); 
	xTaskCreate( vTask2, "Task 2", 1000, NULL, 1, &xTask2Handle );//创建任务2
	for( ;; ) 
	{ 
		/* Print out the name of this task. */ 
		printf( "Task1 is running\r\n" ); 
		/* 把任务2的优先级设置到高于任务1的优先级，会使得任务2立即得到执行(因为任务2现在是所有任务
		中具有最高优先级的任务)。注意调用vTaskPrioritySet()时用到的任务2的句柄。程序清单24将展示
		如何得到这个句柄。 */ 
		printf( "About to raise the Task2 priority\r\n" ); 
		vTaskPrioritySet( xTask2Handle, ( uxPriority +  1 ) ); 
		/* 本任务只会在其优先级高于任务2时才会得到执行。因此，当此任务运行到这里时，任务2必然已经执
		行过了，并且将其自身的优先级设置回比任务1更低的优先级。 */ 		
		vTaskDelay(50);	
	} 
} 
int main( void ) 
{ 
/* Create one task. */ 
	xTaskCreate( vTask1, "Task 1", 1000, NULL, 2, NULL ); 
/* Start the scheduler so our tasks start executing. */ 
	vTaskStartScheduler(); 
/* If all is well then main() will never reach here as the scheduler will 
now be running the tasks. If main() does reach here then it is likely that 
there was insufficient heap memory available for the idle task to be created. 
CHAPTER 5 provides more information on memory management. */ 
	for( ;; ); 
	return 0;
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

