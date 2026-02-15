use std::collections::VecDeque;

enum Stage {
    Initiated,
    Transcoded,
    Stored,
    Ready
}

struct Task {
    stage: Stage,
    buffer: String,
    socket: i64,
}


pub struct EventLoop{
    tasks: VecDeque<Task>,
}

impl EventLoop{
    // init
    pub fn new() -> Self {
        EventLoop { tasks: VecDeque::new()}
    }

    // add tasks
    pub fn add_tasks(&mut self, buffer: impl Into<String>, socket: i64){
        let task:Task = Task {
            stage: Stage::Initiated,
            buffer: buffer.into(),
            socket: socket,
        };
        self.tasks.push_front(task);
    }
    // run
    pub fn run(&mut self){
        while self.tasks.len() > 0 {
            let mut task = self.tasks.pop_back().unwrap();
            match Self::process(&mut task){
                true => {
                    self.tasks.push_front(task);
                }
                false =>{
                    // do nothing;
                }
            }
        }
    }
    //returns whether we need to push back or not
    fn process(task: &mut Task) -> bool{
        match task.stage {
            Stage::Initiated => {
                // process and move to next stage
                // assume we processed
                task.stage = Stage::Transcoded;
                println!("{} Transcoded", task.socket );
                true
            }
            Stage::Transcoded => {
                task.stage = Stage::Stored;
                println!("{} Stored", task.socket );
                true
            }
            Stage::Stored => {
                task.stage = Stage::Ready;
                println!("{} Ready", task.socket );
                true
            }
            Stage::Ready =>{
                false
            }
        }
    }
    
}