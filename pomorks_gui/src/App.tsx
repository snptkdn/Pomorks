import React from 'react';
import logo from './logo.svg';
import './App.css';
import { invoke } from '@tauri-apps/api/tauri'

function App() {
  function executeCommands() {
    //invoke('simple_command')
    invoke('command_with_message', {message: 'some message' }).then(message=>{
      console.log('command_with_message', message)
    })
  }
  return (
    <div className="App">
      <header className="App-header">
        <img src={logo} className="App-logo" alt="logo" />
        <p>
          Edit <code>src/App.tsx</code> and save to reload.

          <br></br> 
            Hello, World!
        </p>

        <button onClick={executeCommands}>Click to exexcute command</button>

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
