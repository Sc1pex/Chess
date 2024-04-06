import { Color, Piece, PieceKind, WasmMove } from "chess-lib";
import { LitElement, css, html } from "lit";
import { customElement, property } from "lit/decorators.js";
import { createRef, ref } from "lit/directives/ref.js";
import { styleMap } from "lit/directives/style-map.js";

@customElement("board-el")
export class BoardEl extends LitElement {
  @property({ type: Array })
  legal_moves: Array<WasmMove> = [];
  @property({ type: Object })
  pieces: Map<number, Piece> = new Map();
  @property({ type: Object })
  handle_move: (move: WasmMove) => void = () => {};
  @property({ type: Boolean })
  flip: boolean = true;

  board_ref = createRef<HTMLDivElement>();
  piece_hover_ref = createRef<HTMLDivElement>();
  promotion_menu_ref = createRef<HTMLDivElement>();
  promote_from: number = -1;
  promote_to: number = -1;
  selected_idx = -1;
  selected_clicked = false;
  dragging = false;

  get_tile_idx(e: MouseEvent): number | null {
    const board_rect = this.board_ref.value?.getBoundingClientRect();
    if (!board_rect) return null;

    const tile_size = board_rect.width / 8;

    const x = e.clientX - board_rect.left;
    const y = e.clientY - board_rect.top;

    const tile_x = Math.floor(x / tile_size);
    const tile_y = this.flip
      ? 7 - Math.floor(y / tile_size)
      : Math.floor(y / tile_size);

    if (tile_x < 0 || tile_x > 7 || tile_y < 0 || tile_y > 7) return null;
    return tile_x + tile_y * 8;
  }

  show_promotion_menu(idx: number) {
    const board_rect = this.board_ref.value?.getBoundingClientRect();
    const tile_size = board_rect!.width / 8;

    const left = board_rect!.left + (idx % 8) * tile_size;
    const top = board_rect!.top + (7 - Math.floor(idx / 8)) * tile_size;

    const menu = this.promotion_menu_ref.value!;
    menu.style.left = `${left}px`;
    menu.style.top = `${top}px`;
    menu.style.width = `${tile_size}px`;
    menu.style.height = `${tile_size * 4}px`;
  }

  hide_promotion_menu() {
    const menu = this.promotion_menu_ref.value!;
    menu.style.left = "-1000px";
    menu.style.top = "-1000px";
  }

  hide_all() {
    this.hide_promotion_menu();
    this.unhighlight_legal_moves();
    this.unhighlight_tile(this.selected_idx);
    this.piece_hover_ref.value!.style.left = "-100px";
    this.piece_hover_ref.value!.style.top = "-100px";
    this.selected_idx = -1;
    this.selected_clicked = false;
    this.promote_from = -1;
    this.promote_to = -1;
  }

  try_move_to(idx: number): boolean {
    const move = this.legal_moves.find(
      (m) => m.to == idx && m.from == this.selected_idx,
    );
    if (move) {
      if (move.promotion !== undefined) {
        this.promote_from = this.selected_idx;
        this.promote_to = idx;
        this.show_promotion_menu(idx);
        return true;
      }

      this.hide_all();
      this.unhighlight_tile(idx);
      this.handle_move(move);
      return true;
    }
    return false;
  }

  mouse_down(e: MouseEvent) {
    if (e.button != 0) return;
    const idx = this.get_tile_idx(e);
    if (idx === null) return;
    this.dragging = true;

    if (this.selected_idx != -1) {
      if (this.try_move_to(idx)) {
        return;
      } else if (this.selected_idx != idx) {
        this.hide_all();
      }
    }

    const p = this.pieces.get(idx);
    if (!p) return;

    this.selected_idx = idx;
    this.highlight_legal_moves(idx);
    this.highlight_tile(idx);

    const tile_width = this.board_ref.value!.getBoundingClientRect().width / 8;
    this.piece_hover_ref.value!.style.width = `${tile_width}px`;
    this.piece_hover_ref.value!.style.height = `${tile_width}px`;

    this.piece_hover_ref.value!.style.backgroundImage = `url(${piece_asset(
      p.kind,
      p.color,
    )})`;
    this.piece_hover_ref.value!.style.left = `${e.clientX - tile_width / 2}px`;
    this.piece_hover_ref.value!.style.top = `${e.clientY - tile_width / 2}px`;
  }

  highlight_legal_moves(idx: number) {
    const tile_size = this.board_ref.value!.getBoundingClientRect().width / 8;

    const legal_moves = this.legal_moves.filter((m) => m.from == idx);
    for (const m of legal_moves) {
      const tile = this.shadowRoot!.getElementById(`move-${m.to}`);
      tile!.style.setProperty("--circle-size", `${tile_size * 0.3}px`);
      if (m.capture) {
        tile!.style.backgroundColor = "rgba(255, 0, 0, 0.5)";
      }
      tile!.style.zIndex = "1";
    }
  }

  unhighlight_legal_moves() {
    for (let i = 0; i < 64; i++) {
      const tile = this.shadowRoot!.getElementById(`move-${i}`);
      tile!.style.zIndex = "-1";
      tile!.style.backgroundColor = "rgba(0, 0, 0, 0.5)";
    }
  }

  highlight_tile(idx: number) {
    const tile = this.shadowRoot!.getElementById(`tile-${idx}`);
    if (tile!.classList.contains("black")) {
      tile!.style.backgroundColor = "#805542";
    } else {
      tile!.style.backgroundColor = "#ccb399";
    }
  }

  unhighlight_tile(idx: number) {
    const tile = this.shadowRoot!.getElementById(`tile-${idx}`);
    if (tile!.classList.contains("black")) {
      tile!.style.backgroundColor = "#a87058";
    } else {
      tile!.style.backgroundColor = "#fbebdb";
    }
  }

  mouse_up(e: MouseEvent) {
    if (e.button != 0) return;

    this.dragging = false;
    this.piece_hover_ref.value!.style.left = "-100px";
    this.piece_hover_ref.value!.style.top = "-100px";

    const idx = this.get_tile_idx(e);
    if (idx === null) return;

    if (this.selected_idx == idx) {
      if (this.selected_clicked) {
        this.hide_all();
      } else {
        this.selected_clicked = !this.selected_clicked;
      }
      return;
    }

    if (this.selected_idx != -1) {
      if (this.try_move_to(idx)) {
        return;
      } else {
        this.hide_all();
        this.unhighlight_tile(idx);
      }
    }
  }

  mouse_move(e: MouseEvent) {
    if (!this.dragging || this.selected_idx == -1) return;
    const tile_width = this.board_ref.value!.getBoundingClientRect().width / 8;
    this.piece_hover_ref.value!.style.left = `${e.clientX - tile_width / 2}px`;
    this.piece_hover_ref.value!.style.top = `${e.clientY - tile_width / 2}px`;
  }

  right_click(e: MouseEvent) {
    e.preventDefault();

    const idx = this.get_tile_idx(e);
    if (idx === null) return;
    this.hide_all();
    this.unhighlight_tile(idx);
  }

  render() {
    return html`
      <div
        class="container"
        @mousedown=${this.mouse_down}
        @mouseup=${this.mouse_up}
        @mousemove=${this.mouse_move}
        @contextmenu=${this.right_click}
      >
        ${this.promotion_menu()}
        <div class="board" ${ref(this.board_ref)}>
          ${Array.from(Array(64).keys()).map((i) => this.board_tile(i))}
        </div>

        <div class="piece-hover" ${ref(this.piece_hover_ref)}></div>
      </div>
    `;
  }

  tile_mouseenter(_e: MouseEvent, idx: number) {
    if (
      this.selected_idx != -1 &&
      this.legal_moves.some((m) => m.to == idx && m.from == this.selected_idx)
    ) {
      this.highlight_tile(idx);
    }
  }

  tile_mouseleave(_e: MouseEvent, idx: number) {
    if (this.selected_idx != -1 && this.selected_idx != idx) {
      this.unhighlight_tile(idx);
    }
  }

  board_tile(i: number) {
    const x = i % 8;
    const y = this.flip ? 7 - Math.floor(i / 8) : Math.floor(i / 8);
    const color = (x + y) % 2 == 0 ? "black" : "white";
    const idx = x + y * 8;

    let piece_style = styleMap({});
    if (this.pieces.has(idx)) {
      const p = this.pieces.get(idx)!;

      piece_style = styleMap({
        "background-image": `url(${piece_asset(p.kind, p.color)})`,
        "background-size": "contain",
      });
    }

    return html`<div
      id="tile-${idx}"
      class="tile ${color}"
      style=${piece_style}
      @mouseenter=${(e: MouseEvent) => this.tile_mouseenter(e, idx)}
      @mouseleave=${(e: MouseEvent) => this.tile_mouseleave(e, idx)}
    >
      <div class="legal_move" id="move-${idx}"></div>
    </div>`;
  }

  handle_promotion(p: PieceKind) {
    const m = this.legal_moves.find(
      (m) =>
        m.from == this.promote_from &&
        m.to == this.promote_to &&
        m.promotion == p,
    );
    if (m) {
      this.handle_move(m);
      this.hide_all();
    }
  }

  promotion_menu() {
    const piece_style = (p: string) =>
      styleMap({
        "background-image": `url(/assets/white_${p}.svg)`,
        "background-size": "contain",
      });

    return html`
      <div class="promotion-menu" ${ref(this.promotion_menu_ref)}>
        <div
          class="tile"
          style=${piece_style("queen")}
          @mousedown=${(e: MouseEvent) => {
            e.stopPropagation();
            this.handle_promotion(PieceKind.Queen);
          }}
        ></div>
        <div
          class="tile"
          style=${piece_style("rook")}
          @mousedown=${(e: MouseEvent) => {
            e.stopPropagation();
            this.handle_promotion(PieceKind.Rook);
          }}
        ></div>
        <div
          class="tile"
          style=${piece_style("bishop")}
          @mousedown=${(e: MouseEvent) => {
            e.stopPropagation();
            this.handle_promotion(PieceKind.Bishop);
          }}
        ></div>
        <div
          class="tile"
          style=${piece_style("knight")}
          @mousedown=${(e: MouseEvent) => {
            e.stopPropagation();
            this.handle_promotion(PieceKind.Knight);
          }}
        ></div>
      </div>
    `;
  }

  static styles = css`
    :host {
      flex: 1 0 auto;
    }

    .tile {
      aspect-ratio: 1;
    }

    .promotion-menu {
      position: absolute;
      top: -1000px;
      left: -1000px;
      background-color: white;
      z-index: 2;
      cursor: pointer;
    }

    .legal_move {
      --circle-size: 20px;
      position: relative;
      top: calc(50% - var(--circle-size) / 2);
      left: calc(50% - var(--circle-size) / 2);
      width: var(--circle-size);
      height: var(--circle-size);
      border-radius: 50%;
      z-index: -1;
      background-color: rgba(0, 0, 0, 0.5);
    }

    .white {
      background-color: #fbebdb;
    }

    .black {
      background-color: #a87058;
    }

    .board {
      display: grid;
      grid-template-columns: repeat(8, 1fr);
      aspect-ratio: 1;
      max-height: 70vh;
      margin: auto;
    }

    .container {
      user-select: none;
    }

    .piece-hover {
      position: absolute;
      width: 64px;
      height: 64px;
      z-index: 1;
      pointer-events: none;
      background-size: contain;
    }
  `;
}

declare global {
  interface HTMLElementTagNameMap {
    "board-el": BoardEl;
  }
}

function piece_asset(p: PieceKind, c: Color): String {
  const color = c == Color.White ? "white" : "black";
  const piece = () => {
    switch (p) {
      case PieceKind.Pawn:
        return "pawn";
      case PieceKind.Knight:
        return "knight";
      case PieceKind.Bishop:
        return "bishop";
      case PieceKind.Rook:
        return "rook";
      case PieceKind.Queen:
        return "queen";
      case PieceKind.King:
        return "king";
    }
  };

  return `/assets/${color}_${piece()}.svg`;
}
