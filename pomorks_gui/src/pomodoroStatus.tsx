export type TYPE_STATE = 
  | "WORK"
  | "BREAK"
  | "LUNCH"
 
export class PomodoroState {
  state: TYPE_STATE;
  workCount: number;

  constructor(state: TYPE_STATE) {
    this.state = state;
    this.workCount = 0;
  }

  // method
  getState(): TYPE_STATE { return this.state; }

  // Todo!:よくない...
  setNextState() {

    if (this.state == "WORK") {
      this.workCount++;
    }

    const isNextLunch = () => {
      return this.state == "WORK" && this.workCount == 4;
    }

    const isNextBreak = () => {
      return this.state == "WORK" && this.workCount != 4;
    }

    if (this.state == "LUNCH" || this.state == "BREAK") {
      this.state = "WORK"
    }
    else if (isNextBreak()) {
      this.state = "BREAK"
    }
    else if (isNextLunch()) {
      this.state = "LUNCH"
    }
    else {
      //Todo!:あさーと的な
    }
  }
}


export function getTimerSeconds(state: TYPE_STATE): number {

  const minutes = 1;
  const workTime : number = minutes * 25;
  const breakTime: number = minutes *  5;
  const lunchTime: number = minutes * 30;

  const seconds = {
    "WORK" : workTime,
    "BREAK" : breakTime,
    "LUNCH" : lunchTime,
  }

  return seconds[state];
}