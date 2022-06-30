#[cfg(test)]
mod tests {
    use pomorks_data_manage::todo::*;
    use std::str::FromStr;
    #[test]
    fn test_add_todo() {
        let mut list = TodoList::new();
        let item = TodoItem::new(
            "1".to_string(),
            "test".to_string(),
            "tag".to_string(),
            "project".to_string(),
            0,
            0,
            "none".to_string(),
        );

        list.add_todo(item).unwrap();
        assert_eq!(list.get_vec_of_todo().len(), 1);

        let item2 = TodoItem::new(
            "1".to_string(),
            "test2".to_string(),
            "tag2".to_string(),
            "project2".to_string(),
            0,
            0,
            "none".to_string(),
        ); // duplicated id

        let result = list.add_todo(item2);
        assert!(result.is_err());
        assert_eq!(list.get_vec_of_todo().len(), 1);
    }

    #[test]
    fn test_delete_todo() {
        let mut list = TodoList::new();

        let todo = TodoItem::from_str(&"test test test 1".to_string()).unwrap();
        list.add_todo(todo.clone()).unwrap();
        list.delete_todo(&todo).unwrap();

        assert_eq!(list.get_vec_of_todo().len(), 0);

        let todo2 = TodoItem::from_str(&"test test test 1".to_string()).unwrap();
        list.add_todo(todo2).unwrap();
        let result = list.delete_todo(&todo.clone()); // no exist.

        assert!(result.is_err());
    }

    #[test]
    fn test_insert_todo() {
        let mut list = TodoList::new();

        let mut todo = TodoItem::from_str(&"test test test 1".to_string()).unwrap();
        list.add_todo(todo.clone()).unwrap();

        todo.estimate_count += 1;
        list.insert_todo(todo).unwrap();

        assert_eq!(list.get_vec_of_todo().len(), 1);
        assert_eq!(list.get_vec_of_todo()[0].estimate_count, 2);
    }

    #[test]
    fn test_drain_finished_todo() {
        let mut list = TodoList::new();
        let status: [bool; 7] = [true, true, false, false, true, false, true];

        status.iter().enumerate().for_each(|(id, finish)| {
            list.add_todo(TodoItem {
                id: id.to_string(),
                title: "test2".to_string(),
                tag: "tag2".to_string(),
                project: "project2".to_string(),
                estimate_count: 0,
                executed_count: 0,
                detail: "none".to_string(),
                finished: *finish,
            })
            .unwrap();
        });
        assert_eq!(list.get_vec_of_todo().len(), 7);

        let list_drained = list.drain_finished_todo();
        assert_eq!(list_drained.len(), 4);
        assert_eq!(list.get_vec_of_todo().len(), 3);
    }
}
