import React from 'react';
import { useState } from 'react';
import { Button, TextField } from '@mui/material';
import AddIcon from '@mui/icons-material/Add';
import { IconButton } from '@mui/material';
import { Grid } from '@mui/material';
import { Rating } from '@mui/material';
import BoltIcon from '@mui/icons-material/Bolt';
import Box from '@mui/material/Box';
import { DataGrid, GridSelectionModel, GridColDef } from '@mui/x-data-grid';
import { Todo } from './Todo';
import DeleteIcon from '@mui/icons-material/Delete';

type Filter = 'all' | 'finished' | 'progress' | 'trash';
type Attribute = 'title' | 'tag' | 'project' | 'pomodoroCount';

export function TodoList({
  todos,
  setTodos,
  emitTargetTodo,
}: {
  todos: Todo[];
  setTodos: (todos: Todo[]) => void;
  emitTargetTodo: (targetTodo: Todo) => void;
}) {
  const DeleteButton = ({ rowId }: { rowId: string }) => {
    const deleteRow = ({ rowId }: { rowId: string }) => {
      const deepCopy = todos.map((todo) => ({ ...todo }));
      const newTodos = deepCopy.filter((todo) => {
        return todo.id !== rowId;
      });

      setTodos(newTodos);
    };

    return (
      <div>
        <IconButton onClick={() => deleteRow({ rowId })}>
          <DeleteIcon style={{ verticalAlign: 'middle', display: 'inline-flex' }}></DeleteIcon>
        </IconButton>
      </div>
    );
  };

  const columns: GridColDef[] = [
    { field: 'title', headerName: 'Title', width: 90 },
    {
      field: 'tag',
      headerName: 'Tag',
      width: 90,
    },
    {
      field: 'project',
      headerName: 'Project',
      width: 90,
    },
    {
      field: 'pomodoro',
      headerName: 'pomodoro',
      width: 90,
    },
    {
      field: 'deleteButton',
      headerName: 'delete',
      sortable: false,
      width: 90,
      renderCell: (params) => <DeleteButton rowId={params.id.toString()}></DeleteButton>,
    },
  ];

  const [todo, setValues] = useState({
    title: '',
    tag: '',
    project: '',
    estimateCount: 0,
    executedCount: 0,
  });
  const [filter, setFilter] = useState<Filter>('all');
  const [pomodoroCount, setCount] = useState<number>(0);
  const [selectionModel, setSelectionModel] = React.useState<GridSelectionModel>([]);

  const handleOnSubmit = () => {
    if (!todo.title) return;

    todo.estimateCount = pomodoroCount;

    const newTodo: Todo = {
      title: todo.title,
      tag: todo.tag,
      project: todo.project,
      estimateCount: todo.estimateCount,
      executedCount: todo.executedCount,
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
      case 'pomodoroCount':
        const value = parseInt(e.target.value);

        setCount(isNaN(value) ? 0 : value);
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
                  inputProps={{ style: { fontSize: 10, color: 'white' } }}
                  InputLabelProps={{ style: { color: 'gray' } }}
                  onChange={(e) => handleOnChange(e, 'title')}
                />
                <TextField
                  margin="normal"
                  label="Tag"
                  value={todo.tag}
                  variant="standard"
                  color="secondary"
                  placeholder="Input New Task"
                  inputProps={{ style: { fontSize: 10, color: 'white' } }}
                  InputLabelProps={{ style: { color: 'gray' } }}
                  onChange={(e) => handleOnChange(e, 'tag')}
                />
                <TextField
                  margin="normal"
                  label="Project"
                  value={todo.project}
                  variant="standard"
                  color="warning"
                  placeholder="Input New Task"
                  inputProps={{ style: { fontSize: 10, color: 'white' } }}
                  InputLabelProps={{ style: { color: 'gray' } }}
                  onChange={(e) => handleOnChange(e, 'project')}
                />
                <IconButton onClick={handleOnSubmit}>
                  <AddIcon style={{ verticalAlign: 'middle', display: 'inline-flex' }}></AddIcon>
                </IconButton>
                <TextField
                  margin="normal"
                  label="Pomodoro"
                  value={pomodoroCount}
                  size="small"
                  variant="standard"
                  color="warning"
                  placeholder="Input Estimate Pomodoro"
                  inputProps={{ style: { fontSize: 10, color: 'white' } }}
                  InputLabelProps={{ style: { color: 'gray' } }}
                  onChange={(e) => handleOnChange(e, 'pomodoroCount')}
                />
                <Rating
                  name="pomodoro-count"
                  value={pomodoroCount}
                  max={10}
                  precision={0.5}
                  icon={<BoltIcon fontSize="inherit" />}
                  emptyIcon={<BoltIcon fontSize="inherit" />}
                  onChange={(e, newValue) => {
                    if (newValue === null) return;
                    setCount(newValue);
                  }}
                />
              </Grid>
            </Grid>
            <Box sx={{ height: 400, width: '95%' }}>
              <DataGrid
                getRowId={(row) => row.id}
                density="compact"
                rows={todos.map((todo) => {
                  return {
                    id: todo.id,
                    title: todo.title,
                    tag: todo.tag,
                    project: todo.project,
                    pomodoro: String(todo.executedCount) + '/' + String(todo.estimateCount),
                  };
                })}
                columns={columns}
                pageSize={5}
                rowsPerPageOptions={[5]}
                autoPageSize
                onSelectionModelChange={(newSelectionModel) => {
                  setSelectionModel(newSelectionModel);
                  const target = todos.find((todo) => {
                    return todo.id === newSelectionModel[0];
                  });
                  if (target !== undefined) {
                    emitTargetTodo(target);
                  }
                }}
                selectionModel={selectionModel}
              />
            </Box>
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
