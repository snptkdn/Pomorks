import React from "react";
import { useTimer } from "use-timer"
import { PomodoroState } from "./pomodoroStatus"
import { TYPE_STATE } from "./pomodoroStatus"
import { getTimerSeconds } from "./pomodoroStatus"
import { getStringOfStatus } from "./pomodoroStatus"

export function PomodoroTimer({state}: {state: PomodoroState}) {
  const {
    time,
    start,
    pause,
    reset,
    status
  } = useTimer({
    initialTime: getTimerSeconds(state.getState()),
    timerType: "DECREMENTAL",
    endTime: 0
  });

  function startPomodoro(state: PomodoroState) {
    start();
    state.setNextState();
    console.log("start pressed. state is ", getStringOfStatus(state.getState(), state.workCount));
  }

  return (
    <div style={{ textAlign: "center" }}>
      <h1>{getStringOfStatus(state.getState(), state.workCount)} </h1>
      <div style={{ fontSize: "100px" }}>
        <span>{Math.floor(time/60)}</span>:<span>{time%60}</span>
      </div>
      <p>{ status === "RUNNING" ? "Process..." : "Done!" } </p>
      <button onClick={() => startPomodoro(state)}>Start</button>
      <button onClick={pause}>Pause</button>
      <button
        onClick={() => {
          const stateStatus = state.getState();
          const timerSeconds = getTimerSeconds(stateStatus);
          const time = new Date();

          time.setSeconds(time.getSeconds() + timerSeconds) // 25minutes
          reset();
        }}
      >
        Restart
      </button>
    </div>
  )
}