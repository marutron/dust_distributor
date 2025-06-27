use std::{collections::VecDeque, vec};

/// Разбивает заданные (в векторе) задачи поровну* между логическими ядрами процессора
pub fn break_tasks_by_cores(overall_tasks_count: u16, num_cpus: u16) -> VecDeque<Vec<u16>> {
    let task_count = overall_tasks_count / num_cpus;

    let mut tasks = VecDeque::new();

    let mut counter = 0u16;
    let mut tasks_vec = vec![];
    if task_count >= 1 {
        for i in 0..overall_tasks_count {
            counter += 1;
            tasks_vec.push(i);
            if counter % task_count == 0 {
                tasks.push_back(tasks_vec.clone());
                tasks_vec = vec![];
            }
        }
        if !tasks_vec.is_empty() {
            tasks.push_back(tasks_vec);
        }
    } else {
        for i in 0..overall_tasks_count {
            tasks.push_back(vec![i]);
        }
    }
    println!("{:#?}", tasks);
    tasks
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // Количество задач кратно числу ядер
    fn tasks_count_multiple_of_cores_num() {
        let accident_duration = 16;
        let num_cpus = 8;
        let res = break_tasks_by_cores(accident_duration, num_cpus);
        assert_eq!(res.len(), 8);
        for vector in res {
            assert_eq!(vector.len(), 2)
        }
    }

    #[test]
    // Количество задач не кратно числу ядер
    fn tasks_count_not_multiple_of_cores_num() {
        let accident_duration = 17;
        let num_cpus = 8;
        let res = break_tasks_by_cores(accident_duration, num_cpus);
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
        let accident_duration = 7;
        let num_cpus = 8;
        let res = break_tasks_by_cores(accident_duration, num_cpus);
        assert_eq!(res.len(), 7);
        for vector in res {
            assert_eq!(vector.len(), 1)
        }
    }
}
