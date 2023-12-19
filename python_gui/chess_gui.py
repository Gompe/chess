import customtkinter
import tkinter

import os, os.path, pathlib
import glob, fnmatch
import re
import subprocess
import collections
import logging

import contextlib

import dataclasses
import functools

import math

import PIL.Image
import PIL.ImageTk

from typing import Iterator, Optional

from . import interface
import time

import enum

# My Imports
from . import chess_types

from .settings import Settings

def create_cell(canvas : customtkinter.CTkCanvas , idx_row : int, idx_col : int) -> int:
    # Cell Color
    parity : int = (idx_row ^ idx_col) & 1
    color : str = Settings.color_black if parity else Settings.color_white

    coordinates : tuple[float, float, float, float] = (
        idx_col * Settings.cell_size, idx_row * Settings.cell_size, 
        (idx_col + 1) * Settings.cell_size, (idx_row + 1) * Settings.cell_size 
    )

    cell_id = canvas.create_rectangle(coordinates, fill=color)

    return cell_id

class TkPiece:
    
    def __init__(self, canvas : customtkinter.CTkCanvas, square : tuple[int, int], piece : chess_types.Piece):
        self.canvas : customtkinter.CTkCanvas = canvas
        self.square : tuple[int, int] = square
        self.piece : chess_types.Piece = piece

        self.img : PIL.ImageTk.PhotoImage = piece.piece_image(width=Settings.cell_size, height=Settings.cell_size)
        self._x : float = self.square[1] * Settings.cell_size
        self._y : float = self.square[0] * Settings.cell_size

        self.piece_id : int = self.canvas.create_image(self._x, self._y, image=self.img, anchor=customtkinter.NW)

    def reset_position(self) -> None:
        self.set_position(self.square[1] * Settings.cell_size, self.square[0] * Settings.cell_size)

    def update_square(self, square : tuple[int, int]) -> None:
        self.square = square
        self.reset_position()

    def get_postion(self) -> tuple[float, float]:
        return self._x, self._y

    def set_position(self, x : float, y : float) -> None:
        self.canvas.move(self.piece_id, x - self._x, y - self._y)
        self.canvas.tag_raise(self.piece_id)

        self._x = x
        self._y = y
    
@dataclasses.dataclass
class CellInfo:
    cell_id : int
    piece_id : Optional[int] = None

    def update_piece_id(self, piece_id : int | None) -> None:
        self.piece_id = piece_id

class TkChessboard:
    
    def __init__(self, master):
        self.canvas : customtkinter.CTkCanvas = customtkinter.CTkCanvas(master=master)
        self.canvas.pack(pady=20, padx=60, fill="both", expand=True)

        self.cell_content : dict[tuple[int, int], CellInfo] = {
            (idx_row, idx_col) : CellInfo(cell_id=create_cell(self.canvas, idx_row, idx_col)) for \
                idx_row, idx_col in TkChessboard.board_iterator()
        }

        self.pieces : dict[int, TkPiece] = dict()

        self.init_pieces()

    def insert_piece(self, square : tuple[int, int], piece : chess_types.Piece) -> None:
        tk_piece = TkPiece(self.canvas, square, piece)

        self.pieces[tk_piece.piece_id] = tk_piece
        self.cell_content[square].update_piece_id(tk_piece.piece_id)

        # Events
        self.canvas.tag_bind(tk_piece.piece_id, '<B1-Motion>', functools.partial(self.on_piece_motion, piece_id=tk_piece.piece_id))
        self.canvas.tag_bind(tk_piece.piece_id, '<ButtonPress-1>', functools.partial(self.on_piece_click, piece_id=tk_piece.piece_id))
        self.canvas.tag_bind(tk_piece.piece_id, '<ButtonRelease-1>', functools.partial(self.on_piece_release, piece_id=tk_piece.piece_id))


    def init_pieces(self) -> None:
        
        for col in range(8):
            self.insert_piece((1, col), chess_types.Piece(chess_types.PieceType.PAWN, False))
            self.insert_piece((6, col), chess_types.Piece(chess_types.PieceType.PAWN, True))

        # Black Pieces
        self.insert_piece((0, 0), chess_types.Piece(chess_types.PieceType.ROOK, False))
        self.insert_piece((0, 1), chess_types.Piece(chess_types.PieceType.KNIGHT, False))
        self.insert_piece((0, 2), chess_types.Piece(chess_types.PieceType.BISHOP, False))
        self.insert_piece((0, 3), chess_types.Piece(chess_types.PieceType.QUEEN, False))
        self.insert_piece((0, 4), chess_types.Piece(chess_types.PieceType.KING, False))
        self.insert_piece((0, 5), chess_types.Piece(chess_types.PieceType.BISHOP, False))
        self.insert_piece((0, 6), chess_types.Piece(chess_types.PieceType.KNIGHT, False))
        self.insert_piece((0, 7), chess_types.Piece(chess_types.PieceType.ROOK, False))

        # White Pieces
        self.insert_piece((7, 0), chess_types.Piece(chess_types.PieceType.ROOK, True))
        self.insert_piece((7, 1), chess_types.Piece(chess_types.PieceType.KNIGHT, True))
        self.insert_piece((7, 2), chess_types.Piece(chess_types.PieceType.BISHOP, True))
        self.insert_piece((7, 3), chess_types.Piece(chess_types.PieceType.QUEEN, True))
        self.insert_piece((7, 4), chess_types.Piece(chess_types.PieceType.KING, True))
        self.insert_piece((7, 5), chess_types.Piece(chess_types.PieceType.BISHOP, True))
        self.insert_piece((7, 6), chess_types.Piece(chess_types.PieceType.KNIGHT, True))
        self.insert_piece((7, 7), chess_types.Piece(chess_types.PieceType.ROOK, True))

    def find_piece(self, piece_id : int) -> tuple[int, int]:
        # To be changed later
        return self.pieces[piece_id].square

    @staticmethod
    def board_iterator() -> Iterator[tuple[int, int]]:
        for idx_row in range(8):
            for idx_col in range(8):
                yield (idx_row, idx_col)
        return

    def on_piece_click(self, event:tkinter.Event, piece_id : int = 0) -> None:
        new_x : float = event.x - self.canvas.canvasx(0) - Settings.cell_size/2
        new_y : float = event.y - self.canvas.canvasy(0) - Settings.cell_size/2
        self.pieces[piece_id].set_position(new_x, new_y)

    def on_piece_motion(self, event:tkinter.Event, piece_id : int = 0) -> None:
        new_x : float = event.x - self.canvas.canvasx(0) - Settings.cell_size/2
        new_y : float = event.y - self.canvas.canvasy(0) - Settings.cell_size/2
        self.pieces[piece_id].set_position(new_x, new_y)

    def on_piece_release(self, event : tkinter.Event, piece_id : int = 0) -> None:
        # Finding release position
        release_x : float = event.x - self.canvas.canvasx(0)
        release_y : float = event.y - self.canvas.canvasy(0)
        
        previous_cell : tuple[int, int] = self.find_piece(piece_id)
        new_cell : tuple[int, int] = int(math.floor(release_y/Settings.cell_size)), int(math.floor(release_x/Settings.cell_size))

        if new_cell[0] < 0 or new_cell[0] > 7 or new_cell[1] < 0 or new_cell[1] > 7:
            return self.pieces[piece_id].reset_position()

        return self.impl_raw_move(previous_cell, new_cell)

    def delete_piece(self, cell : tuple[int, int]) -> None:
        piece_id = self.cell_content[cell].piece_id
        if piece_id is not None:
            self.pieces.pop(piece_id)

    def make_move(self, move : chess_types.Move) -> None:
        if move.is_castle:
            return self.impl_castle_move(move)
        if move.is_en_passant:
            return self.impl_en_passant_move(move)
        if move.is_promotion:
            return self.impl_promotion_move(move)
        
        return self.impl_normal_move(move)

    def impl_raw_move(self, prev_cell : tuple[int, int], new_cell : tuple[int, int]) -> None:
        piece_id = self.cell_content[prev_cell].piece_id

        if prev_cell == new_cell:
            return self.pieces[piece_id].reset_position()
        
        self.delete_piece(new_cell)
        
        if piece_id is not None:
            self.pieces[piece_id].update_square(new_cell)

        self.cell_content[prev_cell].update_piece_id(None)
        self.cell_content[new_cell].update_piece_id(piece_id)

    def impl_normal_move(self, move : chess_types.Move) -> None:
        prev_cell = (move.prev_row, move.prev_col)
        new_cell = (move.new_row, move.new_col)

        return self.impl_raw_move(prev_cell, new_cell)

    def impl_castle_move(self, move : chess_types.Move) -> None:
        if move.is_color_white:
            if move.new_col == 2:
                # Queen Side Castle
                self.impl_raw_move((7, 4), (7, 2))
                self.impl_raw_move((7, 0), (7, 3))
            if move.new_col == 6:
                # King Side Castle
                self.impl_raw_move((7, 4), (7, 6))
                self.impl_raw_move((7, 7), (7, 5))
        else:
            if move.new_col == 2:
                # Queen Side Castle
                self.impl_raw_move((0, 4), (0, 2))
                self.impl_raw_move((0, 0), (0, 3))
            if move.new_col == 6:
                # King Side Castle
                self.impl_raw_move((0, 4), (0, 6))
                self.impl_raw_move((0, 7), (0, 5))

    def impl_promotion_move(self, move : chess_types.Move) -> None:
        prev_cell = (move.prev_row, move.prev_col)
        new_cell = (move.new_row, move.new_col)

        self.impl_raw_move(prev_cell, new_cell)
        self.delete_piece(new_cell)

        self.insert_piece(new_cell, chess_types.Piece(move.promotion_piece, move.is_color_white))

    def impl_en_passant_move(self, move : chess_types.Move) -> None:
        prev_cell = (move.prev_row, move.prev_col)
        new_cell = (move.new_row, move.new_col)

        if move.is_color_white:
            self.delete_piece((move.new_row - 1, move.new_col))
        else:
            self.delete_piece((move.new_row + 1, move.new_col))

        return self.impl_raw_move(prev_cell, new_cell)

import threading

if __name__ == "__main__":
    root = customtkinter.CTk()
    root.geometry(Settings.get_geometry_string())
    chessboard = TkChessboard(root)

    def run_engine():
        time.sleep(1)
        chess_interface = interface.Interface()
        while not chess_interface.is_done():
            move_json = chess_interface.get_message()
            print(move_json, flush=True)

            move = chess_types.json_to_move(move_json)
            chessboard.make_move(move)

            time.sleep(1)
            if move.game_status != 0:
                return

    engine_thread = threading.Thread(target=run_engine)
    engine_thread.start()

    root.mainloop()
