import { Move, PieceKind } from "chess-lib";
import { LitElement, css, html } from "lit";
import { customElement, property } from "lit/decorators.js";
import { createRef, ref } from "lit/directives/ref.js";
import {
  chevron_dleft,
  chevron_dright,
  chevron_left,
  chevron_right,
} from "../icons";

type MovePair = { white: Move; black: Move | undefined };

@customElement("moves-el")
export class MovesEl extends LitElement {
  @property({ type: Array })
  moves: Move[] = [];
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

function move_to_str(m: Move): string {
  if (m.special == "Castle") {
    if (m.to[0] == "G") {
      return "O-O";
    } else {
      return "O-O-O";
    }
  }

  if (m.piece.kind == "Pawn") {
    let move = "";
    if (m.capture) {
      move = `${m.from[0]}x${m.to}`.toLowerCase();
    } else {
      move = `${m.to}`.toLowerCase();
    }

    if (typeof m.special == "object") {
      move += `=${piece_unicode(m.special.Promotion)}`;
    }

    return move;
  }
  let move = piece_unicode(m.piece.kind);
  if (m.capture) {
    move += "x";
  }
  move += `${m.to}`.toLowerCase();

  return move;
}

function piece_unicode(kind: PieceKind): string {
  switch (kind) {
    case "Pawn":
      return "P";
    case "Knight":
      return "N";
    case "Bishop":
      return "B";
    case "Rook":
      return "R";
    case "Queen":
      return "Q";
    case "King":
      return "K";
  }
}

declare global {
  interface HTMLElementTagNameMap {
    "moves-el": MovesEl;
  }
}
