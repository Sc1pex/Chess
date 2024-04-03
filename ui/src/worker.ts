import { WasmBoard, bot_move } from "chess-lib";

export function bot_turn(
  board_json: string,
  ms: bigint = BigInt(3000),
  max_depth: number = 30,
  tt_size: number = 10_000_000,
): string {
  let b = WasmBoard.from_json(board_json);
  let m = bot_move(b, max_depth, tt_size, ms);
  return m.to_json();
}
