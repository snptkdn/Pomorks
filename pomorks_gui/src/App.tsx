import React from 'react';
import './App.css';
import { invoke } from '@tauri-apps/api/tauri'
import { PomodoroTimer } from './Timer'
import { Todo } from './todo'

function App() {
  //function executeCommands() {
    //invoke('command_with_message', {message: 'some message' }).then(message=>{
      //console.log('command_with_message', message)
    //})
  //}

  return (
    <div className="App">
      <header className="App-header">
        <div>
          <PomodoroTimer/>
        </div>

        <Todo/>
      </header>
    </div>
  );
}

export default App;
