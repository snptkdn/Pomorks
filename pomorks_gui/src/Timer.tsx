import React from 'react';
import { Todo } from './Todo';
import { useTimer } from 'use-timer';
import { PomodoroState } from './pomodoroStatus';
import { sendNotification } from '@tauri-apps/api/notification';
import { getTimerSeconds } from './pomodoroStatus';
import { getStringOfStatus } from './pomodoroStatus';
import { IconButton } from '@mui/material';
import PlayCircleFilledWhiteIcon from '@mui/icons-material/PlayCircleFilledWhite';
import PauseCircleIcon from '@mui/icons-material/PauseCircle';
import StopCircleIcon from '@mui/icons-material/StopCircle';

export function PomodoroTimer({
  todos,
  setTodos,
  targetTodo,
}: {
  todos: Todo[];
  setTodos: (todos: Todo[]) => void;
  targetTodo: Todo | undefined;
}) {
  const [state, setState] = React.useState<PomodoroState>(new PomodoroState('WORK', 0));

  const { time, start, pause, reset, status, advanceTime } = useTimer({
    initialTime: getTimerSeconds('BREAK'),
    timerType: 'DECREMENTAL',
    endTime: 0,
    onTimeOver: () => {
      pause();
      sendNotification(getStringOfStatus(state.getState(), state.workCount) + ' is Finish.');
      if (state.getState() === 'WORK' && targetTodo !== undefined) {
        const deepCopy = todos.map((todo) => ({ ...todo }));
        const newTodos = deepCopy.map((todo) => {
          if (todo.id === targetTodo.id) {
            todo.executedCount += 1;
          }
          return todo;
        });

        setTodos(newTodos);
      }
      if (state.getState() === 'BREAK' || state.getState() === 'LUNCH') {
        setState(state.getIncrementWorkCountedState());
      }
      setState(state.getNextState());
      advanceTime(-getTimerSeconds(state.getState()));
    },
  });

  // styleの指定は別ファイルでやりたい。
  return (
    <div style={{ textAlign: 'center' }}>
      <link rel="stylesheet" type="text/css" href="./Timer.css"></link>
      <h1>{getStringOfStatus(state.getState(), state.workCount)} </h1>
      <h1>{targetTodo === undefined ? 'free' : targetTodo.title} </h1>
      <div style={{ fontSize: '100px' }}>
        <span>{Math.floor(time / 60)}</span>:<span>{time % 60}</span>
      </div>
      <p>{status === 'RUNNING' ? 'Process...' : 'Done!'} </p>
      <IconButton onClick={start}>
        <PlayCircleFilledWhiteIcon
          className="Test"
          style={{ fontSize: '48px', color: 'gray' }}
        ></PlayCircleFilledWhiteIcon>
      </IconButton>
      <IconButton onClick={pause}>
        <PauseCircleIcon style={{ fontSize: '48px', color: 'gray' }}></PauseCircleIcon>
      </IconButton>
      <IconButton
        onClick={() => {
          setState(new PomodoroState('BREAK', 0));
          reset();
        }}
      >
        <StopCircleIcon style={{ fontSize: '48px', color: 'gray' }}></StopCircleIcon>
      </IconButton>
    </div>
  );
}
