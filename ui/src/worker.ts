import { WasmBoard, bot_move } from "chess-lib";

export type Difficulty = {
  depth: number;
  ms: bigint;
  tt_size: number;
  worse_move_chance: number;
};

export function bot_turn(board_json: string, difficulty: Difficulty): string {
  let b = WasmBoard.from_json(board_json);
  let m = bot_move(b, difficulty.depth, difficulty.tt_size, difficulty.ms);
  let num = Math.random();
  if (num < difficulty.worse_move_chance) {
    let random_depth = Math.floor(Math.random() * m.depth);
    let mv = m.best_move_at_depth(random_depth);
    return mv.to_json();
  }
  return m.best_move_at_depth(m.depth - 1).to_json();
}
