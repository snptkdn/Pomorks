import React from "react";
import { useState } from "react"
import { TextField } from "@mui/material"
import AddIcon from '@mui/icons-material/Add';
import { IconButton } from "@mui/material"

type Todo = {
  title: string,
  readonly id: string,
  checked: boolean,
  removed: boolean,
}

type Filter = 'all' | 'finished' | 'progress' | 'trash';

export function Todo() {
  const [text, setText] = useState('');
  const [todos, setTodos] = useState<Todo[]>([]);
  const [filter, setFilter] = useState<Filter>('all');

  const handleOnSubmit = () => {
    if (!text) return;

    const newTodo: Todo = {
      title: text,
      id: Math.random().toString(32).substring(2),
      checked: false,
      removed: false,
    };

    setTodos([newTodo, ...todos]);
    setText("");
  }

  const handleOnChange = (e: React.ChangeEvent<HTMLInputElement | HTMLTextAreaElement>) => {
    setText(e.target.value);
  }

  const handleOnEdit = (id: string, value: string) => {
    const deepCopy = todos.map((todo) => ({ ...todo }));
    const newTodos = deepCopy.map((todo) => {
      if (todo.id === id) {
        todo.title = value;
      }
      return todo;
    })

    setTodos(newTodos);
  }

  const handleOnCheck = (id: string, checked: boolean) => {
    const deepCopy = todos.map((todo) => ({ ...todo }));
    const newTodos = deepCopy.map((todo) => {
      if (todo.id === id) {
        todo.checked = !checked;
      }
      return todo;
    })

    setTodos(newTodos);
  }

  const handleOnRemove = (id: string, removed: boolean) => {
    const deepCopy = todos.map((todo) => ({ ...todo }));
    const newTodos = deepCopy.map((todo) => {
      if (todo.id === id) {
        todo.removed = !removed;
      }
      return todo;
    })

    setTodos(newTodos);
  }

  const handleOnEmpty = () => {
    const newTodos = todos.filter((todo) => !todo.removed);
    setTodos(newTodos);
  }

  const filteredTodos = todos.filter((todo) => {
    switch (filter) {
      case 'all':
        return !todo.removed;
      case 'finished':
        return todo.checked && !todo.removed;
      case 'progress':
        return !todo.checked && !todo.removed;
      case 'trash':
        return todo.removed;
      default:
        return todo;
    }
  })


  return (
    <div>
      <select 
        defaultValue="all" 
        onChange={(e) => setFilter(e.target.value as Filter)}
      >
        <option value="all">All Task</option>
        <option value="finished">Finished Task</option>
        <option value="progress">Progress Task</option>
        <option value="trash">Trash</option>
      </select>
      {filter === "trash" ? (
        <button onClick={() => handleOnEmpty()}>
          Remove all.
        </button>
      ) : (
        filter !== "finished" && (
          <form onSubmit={(e) => {
            e.preventDefault()
            handleOnSubmit();
          }}>
            <TextField 
              margin="normal"
              label="NewTask"
              fullWidth
              value={text} 
              placeholder="Input New Task"
              onChange={(e) => handleOnChange(e)} 
            />
            <IconButton onClick={handleOnSubmit}>
              <AddIcon></AddIcon>
            </IconButton>
            </form>
        )
      )}
      <ul>
        {filteredTodos.map((todo) => {
          return (
            <li key={todo.id}>
              <input
                type = "checkbox"
                disabled = {todo.removed}
                checked = {todo.checked}
                onChange={(e) => handleOnCheck(todo.id, todo.checked)}
              />
              <input
                type = "text"
                disabled = {todo.checked || todo.removed}
                value = {todo.title}
                onChange = {(e) => { handleOnEdit(todo.id, e.target.value) }}
              />
              <button onClick={() => handleOnRemove(todo.id, todo.removed)}>
                {todo.removed ? '復元' : '削除' }
              </button>
            </li>
        )})}
      </ul>
    </div>
  );
};