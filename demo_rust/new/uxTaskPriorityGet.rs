fn uxTaskPriorityGet(pxTask:xTaskHandle )->portBASE_TYPE
	{
	let mut pxTCB:&mut tskTCB ;
	let mut uxReturn:portBASE_TYPE;

		taskENTER_CRITICAL();
		{
			/* If null is passed in here then we are changing the
			priority of the calling function. */
			pxTCB = prvGetTCBFromHandle( pxTask );
			uxReturn = pxTCB.uxPriority;
		}
		taskEXIT_CRITICAL();

		uxReturn
	}
