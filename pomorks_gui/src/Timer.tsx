import React from "react";
import { useTimer } from "use-timer"
import { PomodoroState } from "./pomodoroStatus"
import { sendNotification } from '@tauri-apps/api/notification'
import { getTimerSeconds } from "./pomodoroStatus"
import { getStringOfStatus } from "./pomodoroStatus"
let state = new PomodoroState("BREAK", 0);

export function PomodoroTimer() {
  const {
    time,
    start,
    pause,
    reset,
    status,
    advanceTime
  } = useTimer({
    initialTime: getTimerSeconds("BREAK"),
    timerType: "DECREMENTAL",
    endTime: 0,
    onTimeOver: () => {
      pause();
      sendNotification(getStringOfStatus(state.getState(), state.workCount) + " is Finish.");
      state = state.getNextState();
      advanceTime(-getTimerSeconds(state.getState()));
      if (state.getState() === "WORK") {
        state.incrementWorkCount();
      }
    }
  });

  return (
    <div style={{ textAlign: "center" }}>
      <h1>{getStringOfStatus(state.getState(), state.workCount)} </h1>
      <div style={{ fontSize: "100px" }}>
        <span>{Math.floor(time/60)}</span>:<span>{time%60}</span>
      </div>
      <p>{ status === "RUNNING" ? "Process..." : "Done!" } </p>
      <button onClick={start}>Start</button>
      <button onClick={pause}>Pause</button>
      <button
        onClick={() => {
          state = new PomodoroState("BREAK", 0);
          reset();
        }}
      >
        Restart
      </button>
    </div>
  )
}