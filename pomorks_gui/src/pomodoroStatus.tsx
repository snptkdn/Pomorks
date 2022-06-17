export type TYPE_STATE = 'WORK' | 'BREAK' | 'LUNCH';

export class PomodoroState {
  state: TYPE_STATE;
  workCount: number;

  constructor(state: TYPE_STATE, workCount: number) {
    this.state = state;
    this.workCount = workCount;
  }

  // method
  getState(): TYPE_STATE {
    return this.state;
  }

  getIncrementWorkCountedState(): PomodoroState {
    return new PomodoroState(this.getState(), this.workCount + 1);
  }

  // Todo!:よくない...
  getNextState(): PomodoroState {
    const isNextLunch = () => {
      return this.state === 'WORK' && this.workCount === 4;
    };

    const isNextBreak = () => {
      return this.state === 'WORK' && this.workCount !== 4;
    };

    if (this.state === 'LUNCH' || this.state === 'BREAK') {
      return new PomodoroState('WORK', this.workCount);
    } else if (isNextBreak()) {
      return new PomodoroState('BREAK', this.workCount);
    } else if (isNextLunch()) {
      return new PomodoroState('LUNCH', this.workCount);
    } else {
      return new PomodoroState('BREAK', 0);
    }
  }
}

export function getTimerSeconds(state: TYPE_STATE): number {
  const minutes = 0.2;
  const workTime: number = minutes * 25;
  const breakTime: number = minutes * 5;
  const lunchTime: number = minutes * 30;

  const seconds = {
    WORK: workTime,
    BREAK: breakTime,
    LUNCH: lunchTime,
  };

  return seconds[state];
}

export function getStringOfStatus(state: TYPE_STATE, count: number): string {
  const status = {
    WORK: 'WORK_' + String(count),
    BREAK: 'BREAK',
    LUNCH: 'LUNCH',
  };

  return status[state];
}
