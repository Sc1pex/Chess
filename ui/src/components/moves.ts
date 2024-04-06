import { WasmMove, PieceKind, CastleMove } from "chess-lib";
import { LitElement, css, html } from "lit";
import { customElement, property } from "lit/decorators.js";
import { createRef, ref } from "lit/directives/ref.js";
import {
  chevron_dleft,
  chevron_dright,
  chevron_left,
  chevron_right,
} from "../icons";

type MovePair = { white: WasmMove; black: WasmMove | undefined };

@customElement("moves-el")
export class MovesEl extends LitElement {
  @property({ type: Array })
  moves: WasmMove[] = [];
  @property()
  handle_ply_select: (ply: number) => void = () => {};
  @property({ type: Number })
  drawn_ply: number = 0;

  moves_ref = createRef<HTMLDivElement>();

  constructor() {
    super();

    document.addEventListener("keydown", (e) => {
      if (e.key == "ArrowUp") {
        this.handle_ply_select(0);
      }
      if (e.key == "ArrowDown") {
        this.handle_ply_select(this.moves.length);
      }
      if (e.key == "ArrowLeft" && this.drawn_ply > 0) {
        this.handle_ply_select(this.drawn_ply - 1);
      }
      if (e.key == "ArrowRight" && this.drawn_ply < this.moves.length) {
        this.handle_ply_select(this.drawn_ply + 1);
      }
    });
  }

  move_pairs(): MovePair[] {
    let ret = [];
    for (let i = 0; i < this.moves.length; i += 2) {
      ret.push({ white: this.moves[i], black: this.moves[i + 1] });
    }
    return ret;
  }

  render_pair(pair: MovePair, idx: number) {
    return html`
      <div class="idx">${idx + 1}</div>
      <div
        class=${(this.drawn_ply == idx * 2 + 1 ? "selected" : "") + " white"}
        @click=${() => this.handle_ply_select(idx * 2 + 1)}
      >
        ${move_to_str(pair.white)}
      </div>
      <div
        class=${(this.drawn_ply == idx * 2 + 2 ? "selected" : "") + " black"}
        @click=${() => this.handle_ply_select(idx * 2 + 2)}
      >
        ${pair.black ? move_to_str(pair.black) : ""}
      </div>
    `;
  }

  render() {
    return html`
      <div>
        <div class="moves-wrapper" ${ref(this.moves_ref)}>
          <div class="moves">
            ${this.move_pairs().map((pair, i) => this.render_pair(pair, i))}
          </div>
        </div>
        <div class="controls">
          <button class="control" @click=${() => this.handle_ply_select(0)}>
            ${chevron_dleft(30)}
          </button>
          <button
            class="control"
            @click=${() => {
              if (this.drawn_ply > 0) {
                this.handle_ply_select(this.drawn_ply - 1);
              }
            }}
          >
            ${chevron_left(30)}
          </button>
          <button
            class="control"
            @click=${() => {
              if (this.drawn_ply < this.moves.length) {
                this.handle_ply_select(this.drawn_ply + 1);
              }
            }}
          >
            ${chevron_right(30)}
          </button>
          <button
            class="control"
            @click=${() => this.handle_ply_select(this.moves.length)}
          >
            ${chevron_dright(30)}
          </button>
        </div>
      </div>
    `;
  }

  updated() {
    const selected: HTMLDivElement | undefined | null =
      this.shadowRoot?.querySelector(".selected");
    if (selected) {
      this.moves_ref.value?.scrollTo({
        top:
          selected.offsetTop -
          this.moves_ref.value?.offsetTop -
          this.moves_ref.value?.offsetHeight / 2,
      });
    }
  }

  static styles = css`
    @media screen and (orientation: portrait) {
      .moves-wrapper {
        display: none;
      }
    }

    .controls {
      display: flex;
    }

    .control {
      flex: 1 0 25%;
      background-color: #303030;
      border: none;
    }

    .control:hover {
      background-color: #0ea5e9;
      cursor: pointer;
    }

    .moves-wrapper {
      width: 250px;
      background-color: #404040;
      height: 512px;
      max-height: 100%;
      overflow-y: auto;
    }

    .moves {
      display: flex;
      flex-flow: row wrap;
    }

    .white {
      flex: 0 0 42.5%;
      padding: 10px 20px;
      box-sizing: border-box;
    }

    .black {
      flex: 0 0 42.5%;
      padding: 10px 20px;
      box-sizing: border-box;
    }

    .selected {
      background-color: #1e293b;
    }

    .white:hover,
    .black:hover {
      background-color: #0284c7;
      cursor: pointer;
    }

    .idx {
      flex: 0 0 15%;
      background-color: #303030;
      padding: 10px 0px;
      text-align: center;
    }
  `;
}

function idx_to_square(idx: number): string {
  const file = String.fromCharCode(97 + (idx % 8));
  const rank = Math.floor(idx / 8) + 1;
  return file + rank;
}

function move_to_str(m: WasmMove): string {
  if (m.castle !== undefined) {
    if (m.castle == CastleMove.KingSide) {
      return "O-O";
    } else {
      return "O-O-O";
    }
  }

  if (m.piece == PieceKind.Pawn) {
    let move = "";
    if (m.capture) {
      move = `${idx_to_square(m.from)[0]}x${idx_to_square(m.to)}`.toLowerCase();
    } else {
      move = `${idx_to_square(m.to)}`.toLowerCase();
    }

    if (m.promotion !== undefined) {
      move += `=${piece_unicode(m.promotion)}`;
    }

    return move;
  }
  let move = piece_unicode(m.piece);
  if (m.capture) {
    move += "x";
  }
  move += `${idx_to_square(m.to)}`.toLowerCase();

  return move;
}

function piece_unicode(kind: PieceKind): string {
  switch (kind) {
    case PieceKind.Pawn:
      return "P";
    case PieceKind.Knight:
      return "N";
    case PieceKind.Bishop:
      return "B";
    case PieceKind.Rook:
      return "R";
    case PieceKind.Queen:
      return "Q";
    case PieceKind.King:
      return "K";
  }
}

declare global {
  interface HTMLElementTagNameMap {
    "moves-el": MovesEl;
  }
}
