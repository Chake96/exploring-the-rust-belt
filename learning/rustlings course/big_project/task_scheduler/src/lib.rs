

mod task_scheduler{
    mod runner{
        fn run_task(){

        }

        pub fn busy()->bool{
            true
        }
    }

    mod manager{
        pub fn queue_task(){
            if super::runner::busy() == true { //relative path
                println!("Runner is Busy!\n");
                handle_error();
            }

        }

        pub fn remove_task(){

        }

        pub fn clean_tasks(){

        }

        fn handle_error(){
            println!("Handled Error!\n");
        }
    }

    pub fn schedule_task(){
        println!("Scheduling a Task!");
        manager::queue_task();
    }
    
}

pub fn run(){
    //absolute path
    crate::task_scheduler::schedule_task();
    //relative path
    task_scheduler::schedule_task();
    
    //use path
    use task_scheduler::schedule_task;
    schedule_task();

    //alias
    use task_scheduler::schedule_task as st;
    st();
}