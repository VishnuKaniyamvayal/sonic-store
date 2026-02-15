mod storage;
mod runtime;

use runtime::EventLoop;
// use storage::Storage;

fn main(){
    let mut eventLoop = EventLoop::new();
    
    eventLoop.add_tasks("716253716253761253", 123);
    eventLoop.add_tasks("444", 39999);
    eventLoop.add_tasks("716253716253761253", 212123);
    eventLoop.add_tasks("716253716253761253", 1);
    eventLoop.add_tasks("716253716253761253", 2);

    eventLoop.run();

}