import React from 'react';
import './App.css';
import { invoke } from '@tauri-apps/api/tauri'
import { PomodoroTimer } from './Timer'
import { Todo } from './todo'
import { Grid } from '@mui/material'

function App() {
  //function executeCommands() {
    //invoke('command_with_message', {message: 'some message' }).then(message=>{
      //console.log('command_with_message', message)
    //})
  //}

  return (
    <div className="App">
      <header className="App-header">
        <Grid container>
          <Grid item xs={4}>
            <PomodoroTimer/>
          </Grid>
          <Grid item xs={8}>
            <Todo/>
          </Grid>
        </Grid>
      </header>
    </div>
  );
}

export default App;
