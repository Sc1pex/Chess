import {
  WasmBoard,
  Color,
  GameState,
  WasmGame,
  WasmMove,
  opposite_color,
  Difficulty,
} from "chess-lib";
import { LitElement, css, html } from "lit";
import { customElement, property, state } from "lit/decorators.js";
import { createRef, ref } from "lit/directives/ref.js";

const bot_worker = new ComlinkWorker<typeof import("../worker")>(
  new URL("../worker.ts", import.meta.url),
);

function difficulty(d: number) {
  if (d == 0) {
    return Difficulty.Easy;
  } else if (d == 1) {
    return Difficulty.Medium;
  } else {
    return Difficulty.Hard;
  }
}

async function run_worker(board: WasmBoard, diff: number): Promise<WasmMove> {
  const board_json = board.to_json();
  const m = await bot_worker.bot_turn(board_json, difficulty(diff));
  return WasmMove.from_json(m);
}

@customElement("game-el")
export class GameEl extends LitElement {
  @state()
  game: WasmGame = new WasmGame();

  @property()
  game_id: string = "";
  ran_setup: boolean = false;

  @property()
  player_color: string = "white";
  @property({ type: Number })
  difficulty: number = 0;

  drawn_board: WasmBoard = new WasmBoard();
  drawn_histoy: boolean = false;
  drawn_ply: number = 0;

  constructor() {
    super();
    this.drawn_board = this.game.board();
  }

  connectedCallback() {
    super.connectedCallback();

    this.bot_color = this.player_color == "white" ? Color.Black : Color.White;
  }

  bot_color: Color = Color.Black;
  player_moves() {
    if (this.game.side_to_move() == this.bot_color || this.drawn_histoy) {
      return [];
    } else {
      return this.game.legal_moves();
    }
  }

  bot_turn() {
    if (this.game.game_state() != GameState.InProgress) return;

    run_worker(this.game.board(), this.difficulty).then((m) => {
      this.game.make_move(m);
      this.drawn_board = this.game.board();
      this.drawn_histoy = false;
      this.drawn_ply = this.game.move_history().length;
      this.handle_game_state_change();
      this.requestUpdate();
    });
  }

  game_over_div = createRef<HTMLDivElement>();
  game_over_text = createRef<HTMLParagraphElement>();
  handle_game_state_change() {
    this.game.update_state();
    if (this.game.game_state() != GameState.InProgress) {
      if (this.game.game_state() == GameState.Checkmate) {
        this.game_over_text.value!.innerText = `Game Over! ${
          Color[opposite_color(this.game.side_to_move())]
        } wins`;
      }
      if (this.game.game_state() == GameState.Stalemate) {
        this.game_over_text.value!.innerText = "Stalemate!";
      }
      if (this.game.game_state() == GameState.DrawByFiftyMoveRule) {
        this.game_over_text.value!.innerText = "Draw! (50 move rule)";
      }
      if (this.game.game_state() == GameState.DrawByRepetition) {
        this.game_over_text.value!.innerText = "Draw! (Repetition)";
      }
      if (this.game.game_state() == GameState.DrawByInsufficientMaterial) {
        this.game_over_text.value!.innerText = "Draw! (Insufficient Material)";
      }
      this.game_over_div.value!.style.display = "block";

      this.send_game_to_server();
    }
  }

  send_game_to_server() {
    let result = "Draw";
    if (this.game.game_state() == GameState.Checkmate) {
      if (this.game.side_to_move() == this.bot_color) {
        result = "Win";
      } else {
        result = "Loss";
      }
    }

    fetch("/api/submit_game", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        result: result,
        moves: this.game.moves_server(),
        difficulty: this.difficulty,
      }),
    });
  }

  reset() {
    this.game = new WasmGame();
    this.game_over_div.value!.style.display = "none";
    this.drawn_board = this.game.board();
    this.drawn_histoy = false;
    this.drawn_ply = 0;
    this.requestUpdate();
  }

  setup() {
    if (this.ran_setup) return;
    this.ran_setup = true;

    if (this.bot_color == Color.White) {
      setTimeout(() => {
        this.bot_turn();
      }, 2000);
    }

    if (this.game_id !== "") {
      fetch(`/api/game_moves/${this.game_id}`).then((res) => {
        res.text().then((data) => {
          this.game = WasmGame.from_server(data);
          this.drawn_ply = 0;
          this.drawn_board = this.game.board_at(0);
          this.requestUpdate();
        });
      });
    }
  }

  render() {
    this.setup();

    return html` <div class="container">
        <div class="flexrow">
          <board-el
            .pieces=${new Map(
              this.drawn_board.pieces().map((p) => [p.index, p.piece]),
            )}
            .legal_moves=${this.player_moves()}
            .handle_move=${(move: WasmMove) => {
              this.game.make_move(move);
              this.drawn_board = this.game.board();
              this.drawn_histoy = false;
              this.drawn_ply = this.game.move_history().length;
              this.handle_game_state_change();
              this.requestUpdate();

              this.bot_turn();
            }}
            .flip=${this.player_color == "white"}
          ></board-el>
          <moves-el
            .moves=${this.game.move_history()}
            .handle_ply_select=${(idx: number) => {
              this.drawn_board = this.game.board_at(idx);
              this.drawn_ply = idx;
              if (idx == this.game.move_history().length) {
                this.drawn_histoy = false;
              } else {
                this.drawn_histoy = true;
              }
              this.requestUpdate();
            }}
            .drawn_ply=${this.drawn_ply}
          ></moves-el>
        </div>
      </div>
      <div
        class="game-over-bg"
        ${ref(this.game_over_div)}
        @click=${() => {
          this.game_over_div.value!.style.display = "none";
        }}
      >
        <div class="game-over">
          <p ${ref(this.game_over_text)}></p>
          <button class="game-over-button" @click="${this.reset}">
            Play Again
          </button>
          <button class="game-over-button">
            <a href="/" style="color: black; text-decoration: none"
              >Return home</a
            >
          </button>
        </div>
      </div>`;
  }

  static styles = css`
    @media screen and (orientation: portrait) {
      .flexrow {
        flex-direction: column !important;
        align-items: initial !important;
      }
    }

    .flexrow {
      display: flex;
      align-items: center;
      justify-content: center;
      gap: 60px;
      width: 90%;
    }

    .container {
      display: flex;
      justify-content: center;
      flex-direction: column;
      align-items: center;
      height: 100vh;
    }

    .game-over-bg {
      display: none;
      position: absolute;
      top: 0;
      left: 0;
      width: 100%;
      height: 100%;
      background-color: rgba(0, 0, 0, 0.5);
    }

    .game-over {
      position: absolute;
      top: 50%;
      left: 50%;
      transform: translate(-50%, -50%);
      background-color: #18181b;
      padding: 20px;
      border-radius: 10px;
      text-align: center;
    }

    .game-over-button {
      padding: 10px 20px;
      border-radius: 5px;
      background-color: #f0f0f0;
      border: none;
      margin-top: 10px;
      cursor: pointer;
    }
  `;
}

declare global {
  interface HTMLElementTagNameMap {
    "game-el": GameEl;
  }
}
