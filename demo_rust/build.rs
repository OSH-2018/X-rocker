extern crate cc;

fn main() {
    cc::Build::new()
        .file("src/list.c")
        .compile("liblist.a");
    cc::Build::new()
        .file("src/tasks.c")
        .compile("libtasks.a");
    cc::Build::new()
        .file("src/port.c")
        .compile("libport.a");
    cc::Build::new()
        .file("src/queue.c")
        .compile("libqueue.a");        
    cc::Build::new()
        .file("src/heap_3.c")
        .compile("libheap_3.a");    
    cc::Build::new()
        .file("src/other.c")
        .compile("libother.a");              
}