use crate::fork::Fork;
use crate::philosopher::Philosopher;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct Runtime {
    pub philosophers: Vec<Arc<Mutex<Philosopher>>>,
    pub threads: Vec<tokio::task::JoinHandle<()>>,
}

impl Runtime {
    pub fn new(count: u32) -> Runtime {
        let mut philosophers = Vec::new();
        let mut forks_mutex = Vec::new();

        for i in 0..count {
            forks_mutex.push(Mutex::from(Fork::new(i)));
        }

        let mut forks_mutex_arc = vec![];

        for fork in forks_mutex {
            forks_mutex_arc.push(Arc::new(fork));
        }

        for i in 0..usize::try_from(count).unwrap() {
            let philosopher = Philosopher::new(
                i as u32,
                forks_mutex_arc[i].clone(),
                forks_mutex_arc[(i + 1) % usize::try_from(count).unwrap()].clone(),
            );

            philosophers.push(Arc::new(Mutex::new(philosopher)));
        }

        Runtime {
            philosophers,
            threads: vec![],
        }
    }
    pub async fn run(&mut self) {
        for philosopher in self.philosophers.clone() {
            let p = philosopher.clone();
            let handle = tokio::spawn(async move {
                loop {
                    p.lock().await.think().await;
                    p.lock().await.eat().await;
                }
            });

            self.threads.push(handle);
        }
    }

    pub async fn stop(&mut self) {
        for thread in &self.threads {
            thread.abort();
        }

        for p in &self.philosophers {
            let p = p.lock().await;
            println!(
                "Philosopher {}\t\t ate {} ms\t\twaited {} ms\t\tthought {} ms",
                p.id, p.time_eating, p.time_waiting, p.time_thinking
            );
        }
    }
}
