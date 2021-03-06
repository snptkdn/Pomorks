import React from 'react';
import './App.css';
import { invoke } from '@tauri-apps/api/tauri';
import { PomodoroTimer } from './Timer';
import { TodoList } from './TodoList';
import { Todo } from './Todo';
import { Grid } from '@mui/material';

function App() {
  //function executeCommands() {
  //invoke('command_with_message', {message: 'some message' }).then(message=>{
  //console.log('command_with_message', message)
  //})
  //}

  const [targetTodo, setTargetTodo] = React.useState<Todo>();
  const [todos, setTodos] = React.useState<Todo[]>([]);

  return (
    <div className="App">
      <header className="App-header">
        <Grid container>
          <Grid item xs={4}>
            <PomodoroTimer todos={todos} setTodos={setTodos} targetTodo={targetTodo} />
          </Grid>
          <Grid item xs={8}>
            <TodoList todos={todos} setTodos={setTodos} emitTargetTodo={setTargetTodo} />
          </Grid>
        </Grid>
      </header>
    </div>
  );
}

export default App;
