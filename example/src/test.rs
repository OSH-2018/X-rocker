#![crate_type = "staticlib"]
#[no_mangle]
pub extern "C" fn test() {
    print!("HEllo");
}


/*
extern crate libc;
use libc::{c_int};
use std::ffi::CStr;
use std::ffi::CString;
use libc::c_char;
use std::mem;
#[repr(C)]
pub struct tire{
    pub color: c_int,
    pub price: c_int,
} 
impl tire{
    fn new(color:c_int,price:c_int) -> tire {
        tire{
            color : color,
            price : price,
        }
    }
}
#[repr(C)]
pub struct car {
    pub price : c_int,
    pub height : c_int,
	pub a : [tire;4],
	//pub number : [c_char;10],
	pub owner : *mut c_char,
}
impl car{
    pub fn new(price : c_int,height: c_int,owner:&str)->car{
        //let i=0;
        let mut temp:[tire;4]=unsafe {mem::uninitialized()};
        let c_owner=CString::new(owner).unwrap();
        print!("{}",owner);
        for i in 0..4 {
            temp[i]=tire::new(0, 1111);
            println!("{} {}",i,temp[i].price);
        }
        
        car{
            price : price,
            height : height,
            a: temp,
            //number : number,
            owner : c_owner.into_raw(),
        }
    }
}

#[link(name = "ctt",kind="static")]
extern "C" {

//static mut user: *mut c_char;
//static mut version: c_int;
//fn sum(a:c_int,b:c_int) -> c_int;
fn getcar(x:*mut car) -> *mut c_char;
//fn add(a:c_int,b:c_int)-> c_int;
}

fn get_string(raw_string:*mut c_char) -> String {
    unsafe {
        //let raw_string: *mut c_char = char_func();
        print!("ok");
        let cstr = CStr::from_ptr(raw_string);
        cstr.to_string_lossy().into_owned()
    }
}
fn main() {
//let x = unsafe { sum(100,200) };
let mut x = car::new(200,300,"Wang");
let s=unsafe{get_string(getcar(&mut x))};
println!("{}", s);
//print!("{}",unsafe{add(4,5)});
//print!("{}",unsafe{get_string(user)});
}
*/