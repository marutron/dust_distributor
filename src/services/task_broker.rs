use std::{fmt::Debug, sync::Arc, thread, vec};

/// Распределяет переданные в итераторе задачи поровну* между логическими ядрами процессора
/// ### Params
/// task_iter: итератор задач
/// num_cpus: количество доступных логических ядер
pub fn distribute_tasks_by_threads<T, I>(task_iter: I, num_cpus: usize) -> Vec<Vec<T>>
where
    T: Clone + Debug,
    I: IntoIterator<Item = T>,
{
    let overall_task: Vec<T> = task_iter.into_iter().collect();
    let task_by_core = overall_task.len() / num_cpus;

    let mut tasks = vec![];

    let mut counter = 0usize;
    let mut tasks_vec = vec![];

    for i in overall_task {
        if task_by_core > 1 {
            counter += 1;
            tasks_vec.push(i);
            if counter % task_by_core == 0 {
                tasks.push(tasks_vec.clone());
                tasks_vec = vec![];
            }
        } else {
            tasks.push(vec![i]);
        }
    }
    if !tasks_vec.is_empty() {
        tasks.push(tasks_vec);
    }
    tasks
}

/// Получает замыкание и выполняет его на различных потоках с разными входными значениями
/// ### Params
/// tasks_iter: итератор задач (обычно вектор векторов)
/// clos - замыкание-обработчик
pub fn run_into_threads<C, I, T, R>(tasks_iter: I, clos: C) -> Vec<thread::JoinHandle<R>>
where
    I: IntoIterator<Item = T>,
    C: Fn(T) -> R + Send + Sync + 'static,
    T: Send + 'static,
    R: Send + 'static,
{
    let tasks: Vec<T> = tasks_iter.into_iter().collect();
    let clos = Arc::new(clos);
    tasks
        .into_iter()
        .map(|task| {
            let clos = Arc::clone(&clos);
            thread::spawn(move || clos(task))
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
        let res = distribute_tasks_by_threads(initial_task, num_cpus);
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
        let res = distribute_tasks_by_threads(initial_task, num_cpus);
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
        let res = distribute_tasks_by_threads(initial_task, num_cpus);
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
        let res = distribute_tasks_by_threads(initial_task, num_cpus);
        assert_eq!(res.len(), 7);
        println!("{:?}", &res);
        for vector in res {
            assert_eq!(vector.len(), 1)
        }
    }

    #[test]
    // Тест выполнения замыкания в различных потоках
    fn run_into_threads_test() {
        let tasks = [1, 2, 3];
        let closure = |_| thread::current().id();
        let main_th_id = thread::current().id();

        let handles = run_into_threads(tasks, closure);
        for handle in handles {
            assert_ne!(main_th_id, handle.join().unwrap());
        }
    }
}
