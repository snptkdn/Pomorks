import React from "react";
import { useState } from "react"

type Todo = {
  title: string,
  readonly id: string,
  checked: boolean;
}

export function Todo() {
  const [text, setText] = useState('');
  const [todos, setTodos] = useState<Todo[]>([]);

  const handleOnSubmit = () => {
    if (!text) return;

    const newTodo: Todo = {
      title: text,
      id: Math.random().toString(32).substring(2),
      checked: false,
    };

    setTodos([newTodo, ...todos]);
    setText("");
  }

  const handleOnChange = (e: React.ChangeEvent<HTMLInputElement>) => {
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


  return (
    <div>
      <form onSubmit={(e) => {
        e.preventDefault()
        handleOnSubmit();
      }}
      >
        <input 
          type="text" 
          value={text} 
          onChange={(e) => handleOnChange(e)} 
        />
        <input
          type="submit"
          value="追加"
          onSubmit={handleOnSubmit}
        />
      </form>
      <ul>
        {todos.map((todo) => {
          return (
            <li key={todo.id}>
              <input
                type = "checkbox"
                checked = {todo.checked}
                onChange={(e) => handleOnCheck(todo.id, todo.checked)}
              />
              <input
                type = "text"
                disabled = {todo.checked}
                value = {todo.title}
                onChange = {(e) => { handleOnEdit(todo.id, e.target.value) }}
              />
            </li>
        )})}
      </ul>
    </div>
  );
};