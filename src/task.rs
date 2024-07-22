pub struct TaskQueue {
    tasks: Vec<Box<dyn FnMut() + 'static>>,
}

impl TaskQueue {
    pub fn new() -> Self {
        TaskQueue { tasks: Vec::new() }
    }

    pub fn add_task<F>(&mut self, task: F)
    where
        F: FnMut() + 'static,
    {
        self.tasks.push(Box::new(task))
    }

    pub fn run_tasks(&mut self) {
        while let Some(task) = &mut self.tasks.pop() {
            task();
        }
    }
}
