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

use std::*;
const Prioritymax:u32=256;
//static mut a: Vec<xNode>::new();//新建一个向量存储所有的资源(使用资源时必须在unsafe块中)
/*
struct xListItem{
	xItemValue:	i32;
	list:	LinkedList<>
}*/
struct Item{
	xItemValue:	i32,
	pvOwner:	usize,//索引
}
enum xNode{
	xItem(Item),
	xTCB(tskTCB),
	NULL,
}
struct tskTCB {
	xGenericListItem: usize,//索引
	xEventListItem: usize,//索引
	uxPriority: u32,
	pxStack: Box<Vec<i32>>,
	pcTaskName: String,
	placenum: usize
}//任务的控制块
impl tskTCB{
	fn new(a:&mut Vec<xNode>,StackDepth:usize) -> usize{
		let b=vec![0;StackDepth];//创建一个向量，作为栈使用
		let x=lookforindex(a);
		let t=tskTCB{
			xGenericListItem:	0,
			xEventListItem:		0,
			uxPriority:			0,//优先级
			pxStack:			Box::new(b),//传输向量的装箱（类似指针）
			pcTaskName:			String::new(),//任务名称
			placenum:			x
		};//创建控制块		
		a[x]=xNode::xTCB(t);//将资源所有权移交给a[x]
		x				
	}//任务控制块的创建	
}
fn TaskCreate(){

}
fn init(x:usize,a:&mut Vec<xNode>,pcTaskName:&str,uxPriority:u32){
	let mut place:usize =0;
	let mut pri:u32=0;//创建item所需的参数
	if let xNode::xTCB(ref mut t) =a[x]{
		t.pcTaskName.push_str(pcTaskName);//设置任务名称
		let x:u32;
		if uxPriority>=Prioritymax{
			x=Prioritymax-1;
		}
		else {
			x=uxPriority;
		}
		t.uxPriority=x;//设置优先级
		place=t.placenum;
		pri=x;//传递参数
	}

	let list1=Item{
		xItemValue:0,
		pvOwner:place
	};//创建xGenericListItem
	let temp1=lookforindex(a);//暂存索引以便记录到TCB中
	a[temp1]=xNode::xItem(list1);//移交所有权
		
	let list2=Item{
		xItemValue: (Prioritymax-pri) as i32,
		pvOwner:place
	};//创建xEventListItem
	let temp2=lookforindex(a);//暂存索引以便记录到TCB中
	a[temp2]=xNode::xItem(list2);//移交所有权

	if let xNode::xTCB(ref mut t) =a[x]{
		t.xGenericListItem=temp1;//记录索引
		t.xEventListItem=temp2;//记录索引
	}
}
fn lookforindex(a: &mut Vec<xNode>)->usize{
	let mut i:usize =0;
	for i in 0..a.len() {
		if let xNode::NULL = a[i] {
			break;
		}
	}
	if i>=a.len() {
		a.push(xNode::NULL);
	}
	i
}//找到一个空闲的资源列表索引
/*
impl xNode{
	fn getTCB(&mut self)-> &mut tskTCB{
		let mut b=tskTCB::fakenew();
		if let xNode::xTCB(ref mut h) = *self {
			return h;
		}
		else{
			return 0 as &mut tskTCB;
		}
		/*let head= match *self{
			xNode::xTCB(ref mut h) => h,
			xNode::NULL => &mut b,
			xNode::xItem(h) => &mut b,
		};
		head*/
	}
}*/
fn main(){
	let mut a:Vec<xNode>=Vec::new();//资源列表，统一管理资源
	//let mut b=tskTCB::fakenew();
	let mut x=tskTCB::new(&mut a,10);
	//let mut head: &mut tskTCB;
/*	
	let head= match a[x]{
		xNode::xTCB(ref mut h) => h,
		_=> &mut b,
	};
*/	
	//let head=a[x].getTCB();
	//println!("{}",head.uxPriority);
	//head.pxStack[1]=1;
	//println!("{}",head.pxStack[1]);
	init(x,& mut a,"Hello",32);
	//println!("{}",head.pcTaskName);
	//println!("{}",head.uxPriority);
	
	
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