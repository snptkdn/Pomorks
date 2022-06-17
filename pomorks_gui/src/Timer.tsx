import React from 'react';
import { Todo } from './todo';
import { useTimer } from 'use-timer';
import { PomodoroState } from './pomodoroStatus';
import { sendNotification } from '@tauri-apps/api/notification';
import { getTimerSeconds } from './pomodoroStatus';
import { getStringOfStatus } from './pomodoroStatus';
import { IconButton } from '@mui/material';
import PlayCircleFilledWhiteIcon from '@mui/icons-material/PlayCircleFilledWhite';
import PauseCircleIcon from '@mui/icons-material/PauseCircle';
import StopCircleIcon from '@mui/icons-material/StopCircle';

let state = new PomodoroState('BREAK', 0);

export function PomodoroTimer({ targetTodo }: { targetTodo: Todo | undefined }) {
  const { time, start, pause, reset, status, advanceTime } = useTimer({
    initialTime: getTimerSeconds('BREAK'),
    timerType: 'DECREMENTAL',
    endTime: 0,
    onTimeOver: () => {
      pause();
      sendNotification(getStringOfStatus(state.getState(), state.workCount) + ' is Finish.');
      state = state.getNextState();
      advanceTime(-getTimerSeconds(state.getState()));
      if (state.getState() === 'WORK') {
        state.incrementWorkCount();
      }
    },
  });

  // styleの指定は別ファイルでやりたい。
  return (
    <div style={{ textAlign: 'center' }}>
      <link rel="stylesheet" type="text/css" href="./Timer.css"></link>
      <h1>{getStringOfStatus(state.getState(), state.workCount)} </h1>
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
          state = new PomodoroState('BREAK', 0);
          reset();
        }}
      >
        <StopCircleIcon style={{ fontSize: '48px', color: 'gray' }}></StopCircleIcon>
      </IconButton>
    </div>
  );
}
