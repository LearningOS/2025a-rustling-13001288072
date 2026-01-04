use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct JobStatus {
    jobs_completed: u32,
}

fn main() {
    // 1. 用Mutex包装JobStatus，保证线程安全的修改；外层用Arc实现多线程共享所有权
    let status = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 }));
    let mut handles = vec![];
    
    for _ in 0..10 {
        let status_shared = Arc::clone(&status);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));
            // 2. 锁定Mutex（获取互斥锁），才能安全修改共享值
            let mut status = status_shared.lock().unwrap();
            status.jobs_completed += 1;
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
        // 3. 读取共享值时也需要锁定Mutex，填入status.lock().unwrap().jobs_completed
        println!("jobs completed {}", status.lock().unwrap().jobs_completed);
    }
}
