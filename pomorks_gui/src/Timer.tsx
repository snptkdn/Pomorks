import React from "react";
import { useTimer } from "react-timer-hook"
import { PomodoroState } from "./pomodoroStatus"
import { TYPE_STATE } from "./pomodoroStatus"
import { getTimerSeconds } from "./pomodoroStatus"

export function PomodoroTimer() {
  const state: PomodoroState = new PomodoroState("WORK");
  const stateStatus = state.getState();
  const timerSeconds = getTimerSeconds(stateStatus);
  const expiryTimestamp = new Date();
  expiryTimestamp.setSeconds(getTimerSeconds(state.getState()));
  const {
    seconds,
    minutes,
    isRunning,
    start,
    pause,
    resume,
    restart,
  } = useTimer({
    expiryTimestamp,
    onExpire: () => console.warn("onExpire called"),
  });

  function startPomodoro(state: PomodoroState) {
    state.setNextState();
    return start();
  }

  return (
    <div style={{ textAlign: "center" }}>
      <h1>Pomorks</h1>
      <div style={{ fontSize: "100px" }}>
        <span>{minutes}</span>:<span>{seconds}</span>
      </div>
      <p>{ isRunning ?  "Process..." : "Done!" } </p>
      <button onClick={() => startPomodoro(state)}>Start</button>
      <button onClick={pause}>Pause</button>
      <button onClick={resume}>Resume</button>
      <button
        onClick={() => {
          const stateStatus = state.getState();
          const timerSeconds = getTimerSeconds(stateStatus);
          const time = new Date();

          time.setSeconds(time.getSeconds() + timerSeconds) // 25minutes
          restart(time);
        }}
      >
        Restart
      </button>
    </div>
  )
}