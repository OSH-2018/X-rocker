//struct xListItem
//{
//	xItemValue : i32,				/*< The value being listed.  In most cases this is used to sort the list in descending order. */
//    pxNext: struct xLIST_ITEM ,	/*< Pointer to the next xListItem in the list. */
//	volatile struct xLIST_ITEM * pxPrevious;/*< Pointer to the previous xListItem in the list. */
//	void * pvOwner;							/*< Pointer to the object (normally a TCB) that contains the list item.  There is therefore a two way link between the object containing the list item and the list item itself. */
//	void * pvContainer;						/*< Pointer to the list in which this list item is placed (if any). */
//}
//use List::*;\
/*
use std::*;
#[derive(Debug)]
struct ListItem {
	value: i32,
	next: Box<List>,
	pre: Box<List>,
}
#[derive(Debug)]
enum List {
	Cons(ListItem), 
	NULL,
}
impl List {
	fn new() ->List{
		List::NULL
	}
	fn prepend(mut self,elem:i32) -> List {
		//self::Cons.pre=
		let t:List =List::Cons(ListItem{value:elem,next:Box::new(self),pre:Box::new(List::NULL)});
		if let List::Cons(ref mut x)= self {
			x.pre=Box::new(t);
		}		
		t
	}
	fn stringify(&self) -> String {
        let mut t = self;
        let mut res = String::new();
        while let List::Cons(ref x)= *t {
            res.push_str(&format!("{}==>", x.value));
         	t = &(x.next);
        }
        res.push_str(&format!("NULL"));
        res
    }
}
//typedef struct xLIST_ITEM xListItem;		/* For some reason lint wants this as two separate definitions. */
*/

enum xListItem{
	cons(i32),
	NULL,
}

struct tskTCB {
	xGenericListItem: xListItem,
	xEventListItem: xListItem,
	uxPriority: u32,
	pxStack: Box<Vec<i32>>,
	pcTaskName: String,
}//任务的控制块
impl tskTCB{
	fn new(StackDepth:usize) -> tskTCB{
		let mut a=vec![0;StackDepth];//创建一个向量，作为栈使用
		tskTCB{
			xGenericListItem:	xListItem::NULL,
			xEventListItem:		xListItem::NULL,
			uxPriority:			0,
			pxStack:			Box::new(a),//传输向量的装箱（类似指针）
			pcTaskName:			String::new()
		}		
	}//任务控制块的创建
	fn initial(&mut self,pcTaskName:&str,uxPriority:u32,StackDepth:usize){
			let mut t=self;
			t.pcTaskName.push_str(pcTaskName);
	}
}
fn TaskCreate(){

}

fn main(){
	let mut head=tskTCB::new(10);
	println!("{}",head.uxPriority);
	head.pxStack[1]=1;
	println!("{}",head.pxStack[1]);
	head.initial("Hello",0,10);
	println!("{}",head.pcTaskName);
}
/*
struct tskTaskControlBlock
{
	volatile portSTACK_TYPE	*pxTopOfStack;		/*< Points to the location of the last item placed on the tasks stack.  THIS MUST BE THE FIRST MEMBER OF THE STRUCT. */

	#if ( portUSING_MPU_WRAPPERS == 1 )
		xMPU_SETTINGS xMPUSettings;				/*< The MPU settings are defined as part of the port layer.  THIS MUST BE THE SECOND MEMBER OF THE STRUCT. */
	#endif	
	
	xListItem				xGenericListItem;	/*< List item used to place the TCB in ready and blocked queues. */
	xListItem				xEventListItem;		/*< List item used to place the TCB in event lists. */
	unsigned portBASE_TYPE	uxPriority;			/*< The priority of the task where 0 is the lowest priority. */
	portSTACK_TYPE			*pxStack;			/*< Points to the start of the stack. */
	signed char				pcTaskName[ configMAX_TASK_NAME_LEN ];/*< Descriptive name given to the task when created.  Facilitates debugging only. */

	#if ( portSTACK_GROWTH > 0 )
		portSTACK_TYPE *pxEndOfStack;			/*< Used for stack overflow checking on architectures where the stack grows up from low memory. */
	#endif

	#if ( portCRITICAL_NESTING_IN_TCB == 1 )
		unsigned portBASE_TYPE uxCriticalNesting;
	#endif

	#if ( configUSE_TRACE_FACILITY == 1 )
		unsigned portBASE_TYPE	uxTCBNumber;	/*< This stores a number that increments each time a TCB is created.  It allows debuggers to determine when a task has been deleted and then recreated. */
		unsigned portBASE_TYPE  uxTaskNumber;	/*< This stores a number specifically for use by third party trace code. */
	#endif

	#if ( configUSE_MUTEXES == 1 )
		unsigned portBASE_TYPE uxBasePriority;	/*< The priority last assigned to the task - used by the priority inheritance mechanism. */
	#endif

	#if ( configUSE_APPLICATION_TASK_TAG == 1 )
		pdTASK_HOOK_CODE pxTaskTag;
	#endif

	#if ( configGENERATE_RUN_TIME_STATS == 1 )
		unsigned long ulRunTimeCounter;		/*< Used for calculating how much CPU time each task is utilising. */
	#endif

} tskTCB;
*/