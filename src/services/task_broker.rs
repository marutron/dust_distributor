use std::{collections::VecDeque, fmt::Debug, sync::Arc, thread, vec};

/// Разбивает заданные (в векторе) задачи поровну* между логическими ядрами процессора
pub fn break_tasks_by_cores<T, I>(task_iter: I, num_cpus: usize) -> VecDeque<Vec<T>>
where
    T: Clone + Debug,
    I: IntoIterator<Item = T>,
{
    let overall_task: Vec<T> = task_iter.into_iter().collect();
    let task_by_core = overall_task.len() / num_cpus;

    let mut tasks = VecDeque::new();

    let mut counter = 0usize;
    let mut tasks_vec = vec![];

    for i in overall_task {
        if task_by_core > 1 {
            counter += 1;
            tasks_vec.push(i);
            if counter % task_by_core == 0 {
                tasks.push_back(tasks_vec.clone());
                tasks_vec = vec![];
            }
        } else {
            tasks.push_back(vec![i]);
        }
    }
    if !tasks_vec.is_empty() {
        tasks.push_back(tasks_vec);
    }
    tasks
}

pub fn run_in_threads<F, I, T, R>(tasks_iter: I, f: F) -> Vec<thread::JoinHandle<R>>
where
    F: Fn(T) -> R + Send + Sync + 'static,
    I: IntoIterator<Item = T>,
    T: Send + 'static,
    R: Send + 'static,
{
    let tasks: Vec<T> = tasks_iter.into_iter().collect();
    let f = Arc::new(f);
    tasks
        .into_iter()
        .map(|task| {
            let f = Arc::clone(&f);
            thread::spawn(move || f(task))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // Количество задач кратно числу ядер
    fn tasks_count_equals_of_cores_num() {
        let initial_task = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let num_cpus = 8;
        let res = break_tasks_by_cores(initial_task, num_cpus);
        assert_eq!(res.len(), 8);
        for vector in res {
            assert_eq!(vector.len(), 1)
        }
    }

    #[test]
    // Количество задач кратно числу ядер
    fn tasks_count_multiple_of_cores_num() {
        let initial_task = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
        let num_cpus = 8;
        let res = break_tasks_by_cores(initial_task, num_cpus);
        assert_eq!(res.len(), 8);
        for vector in res {
            assert_eq!(vector.len(), 2)
        }
    }

    #[test]
    // Количество задач не кратно числу ядер
    fn tasks_count_not_multiple_of_cores_num() {
        let initial_task = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17];
        let num_cpus = 8;
        let res = break_tasks_by_cores(initial_task, num_cpus);
        assert_eq!(res.len(), 9);
        for i in 0..res.len() {
            let vector = &res[i];
            if i == res.len() - 1 {
                assert_eq!(vector.len(), 1)
            } else {
                assert_eq!(vector.len(), 2)
            }
        }
    }

    #[test]
    // Количество задач меньше числа ядер
    fn tasks_count_less_then_cores_num() {
        let initial_task = vec![1, 2, 3, 4, 5, 6, 7];
        let num_cpus = 8;
        let res = break_tasks_by_cores(initial_task, num_cpus);
        assert_eq!(res.len(), 7);
        println!("{:?}", &res);
        for vector in res {
            assert_eq!(vector.len(), 1)
        }
    }
}
