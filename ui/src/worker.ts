import { Difficulty, WasmBoard, bot_move } from "chess-lib";

export function bot_turn(board_json: string, difficulty: Difficulty): string {
  let b = WasmBoard.from_json(board_json);
  let m = bot_move(b, difficulty);
  return m.best_move.to_json();
}
