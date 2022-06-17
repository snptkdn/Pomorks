export type Todo = {
  title: string;
  tag: string;
  project: string;
  readonly id: string;
  checked: boolean;
  removed: boolean;
  estimateCount: number;
  executedCount: number;
};
