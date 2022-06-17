import React from 'react';
import { useState } from 'react';
import { TextField } from '@mui/material';
import AddIcon from '@mui/icons-material/Add';
import { IconButton } from '@mui/material';
import { Grid } from '@mui/material';
import { height } from '@mui/system';

type Todo = {
  title: string;
  tag: string;
  project: string;
  readonly id: string;
  checked: boolean;
  removed: boolean;
};

type Filter = 'all' | 'finished' | 'progress' | 'trash';
type Attribute = 'title' | 'tag' | 'project';

export function Todo() {
  const [todo, setValues] = useState({
    title: '',
    tag: '',
    project: '',
  });
  const [todos, setTodos] = useState<Todo[]>([]);
  const [filter, setFilter] = useState<Filter>('all');

  const handleOnSubmit = () => {
    if (!todo.title) return;

    const newTodo: Todo = {
      title: todo.title,
      tag: todo.tag,
      project: todo.project,
      id: Math.random().toString(32).substring(2),
      checked: false,
      removed: false,
    };

    setTodos([newTodo, ...todos]);
    setValues({ ...todo, title: '' });
  };

  const handleOnChange = (
    e: React.ChangeEvent<HTMLInputElement | HTMLTextAreaElement>,
    attr: Attribute,
  ) => {
    switch (attr) {
      case 'title':
        setValues({ ...todo, title: e.target.value });
        break;
      case 'tag':
        setValues({ ...todo, tag: e.target.value });
        break;
      case 'project':
        setValues({ ...todo, project: e.target.value });
        break;
    }
  };

  const handleOnEdit = (id: string, value: string) => {
    const deepCopy = todos.map((todo) => ({ ...todo }));
    const newTodos = deepCopy.map((todo) => {
      if (todo.id === id) {
        todo.title = value;
      }
      return todo;
    });

    setTodos(newTodos);
  };

  const handleOnCheck = (id: string, checked: boolean) => {
    const deepCopy = todos.map((todo) => ({ ...todo }));
    const newTodos = deepCopy.map((todo) => {
      if (todo.id === id) {
        todo.checked = !checked;
      }
      return todo;
    });

    setTodos(newTodos);
  };

  const handleOnRemove = (id: string, removed: boolean) => {
    const deepCopy = todos.map((todo) => ({ ...todo }));
    const newTodos = deepCopy.map((todo) => {
      if (todo.id === id) {
        todo.removed = !removed;
      }
      return todo;
    });

    setTodos(newTodos);
  };

  const handleOnEmpty = () => {
    const newTodos = todos.filter((todo) => !todo.removed);
    setTodos(newTodos);
  };

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
  });

  return (
    <div>
      {filter === 'trash' ? (
        <button onClick={() => handleOnEmpty()}>Remove all.</button>
      ) : (
        filter !== 'finished' && (
          <form
            onSubmit={(e) => {
              e.preventDefault();
              handleOnSubmit();
            }}
          >
            <Grid container>
              <Grid item xs={11}>
                <TextField
                  margin="normal"
                  label="NewTask"
                  value={todo.title}
                  variant="standard"
                  placeholder="Input New Task"
                  inputProps={{ style: { fontSize: 10 } }}
                  onChange={(e) => handleOnChange(e, 'title')}
                />
                <TextField
                  margin="normal"
                  label="Tag"
                  value={todo.tag}
                  variant="standard"
                  placeholder="Input New Task"
                  inputProps={{ style: { fontSize: 10 } }}
                  onChange={(e) => handleOnChange(e, 'tag')}
                />
                <TextField
                  margin="normal"
                  label="Project"
                  value={todo.project}
                  variant="standard"
                  placeholder="Input New Task"
                  inputProps={{ style: { fontSize: 10 } }}
                  onChange={(e) => handleOnChange(e, 'project')}
                />
                <IconButton onClick={handleOnSubmit}>
                  <AddIcon style={{ verticalAlign: 'middle', display: 'inline-flex' }}></AddIcon>
                </IconButton>
              </Grid>
            </Grid>
          </form>
        )
      )}
      <select defaultValue="all" onChange={(e) => setFilter(e.target.value as Filter)}>
        <option value="all">All Task</option>
        <option value="finished">Finished Task</option>
        <option value="progress">Progress Task</option>
        <option value="trash">Trash</option>
      </select>
      <ul>
        {filteredTodos.map((todo) => {
          return (
            <li key={todo.id}>
              <input
                type="checkbox"
                disabled={todo.removed}
                checked={todo.checked}
                onChange={(e) => handleOnCheck(todo.id, todo.checked)}
              />
              <input
                type="text"
                disabled={todo.checked || todo.removed}
                value={todo.title}
                onChange={(e) => {
                  handleOnEdit(todo.id, e.target.value);
                }}
              />
              <button onClick={() => handleOnRemove(todo.id, todo.removed)}>
                {todo.removed ? '復元' : '削除'}
              </button>
            </li>
          );
        })}
      </ul>
    </div>
  );
}
