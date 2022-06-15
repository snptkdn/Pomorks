import React from 'react';
import './App.css';
import { invoke } from '@tauri-apps/api/tauri'
import { open } from '@tauri-apps/api/dialog'
import { sendNotification } from '@tauri-apps/api/notification'
import { PomodoroTimer } from './Timer'
import { PomodoroState } from './pomodoroStatus'

function App() {
  function sendTime () {
    sendNotification('Time is up.')
  }
  function openDialog () {
    open().then(files => console.log(files))
  }
  function executeCommands() {
    invoke('command_with_message', {message: 'some message' }).then(message=>{
      console.log('command_with_message', message)
    })
  }

  const state: PomodoroState = new PomodoroState("BREAK");
  return (
    <div className="App">
      <header className="App-header">
        <div>
          <PomodoroTimer state={state}/>
        </div>

        <button onClick={executeCommands}>Click to exexcute command</button>
        <button onClick={sendTime}>Click to open Dialog</button>

        <a
          className="App-link"
          href="https://reactjs.org"
          target="_blank"
          rel="noopener noreferrer"
        >
          Learn React
        </a>
      </header>
    </div>
  );
}

export default App;
