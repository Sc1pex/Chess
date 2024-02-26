import {
  Board,
  Color,
  PieceKind,
  SpecialMove,
  Square,
  bot_move,
} from "chess-lib";

type WorkerMove = {
  from: Square;
  to: Square;
  piece: {
    kind: PieceKind;
    color: Color;
  };
  capture: boolean;
  special?: SpecialMove;
};

export function bot_turn(board: any): WorkerMove {
  let b = Board.from_js_value(board);
  let m = bot_move(b);

  return {
    from: m.from,
    to: m.to,
    piece: {
      kind: m.piece.kind,
      color: m.piece.color,
    },
    capture: m.capture,
    special: m.special,
  };
}
