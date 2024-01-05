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

import time

import enum

import pathlib
import subprocess
import sys

from typing import Protocol

import select

from .chess_gui import TkChessboard
from .settings import Settings
from .chess_types import Move, PieceType 

PATH_EXECUTABLE = "/home/cabral/RustProjects/chess/target/release/chess"

class PlayerInterface(Protocol):

    def start(self, color: str) -> None:
        ...

    def receive_move(self, move: str) -> None:
        ...

    def emit_move(self) -> Optional[str]:
        ...

class RustProcessInterface:

    def __init__(self, path_executable: pathlib.Path) -> None:
        self.path_executable = path_executable

        
    def start(self, color: str) -> None:
        
        print("Starting player:", color)
        
        self.proc = subprocess.Popen(
            self.path_executable, 
            stdin=subprocess.PIPE,
            stdout=subprocess.PIPE, 
            stderr=subprocess.PIPE,
            universal_newlines=True
        )
        
        color = f"{color}\n"
        self.proc.stdin.write(color)
        self.proc.stdin.flush()

    def receive_move(self, move: str) -> None:
        
        string = move
         
        print("Receive move string", string, flush=True)
        
        string = f"{string}\n"
        
        self.proc.stdin.write(string)
        self.proc.stdin.flush()
        

    def emit_move(self) -> Optional[str]:
        print("Emit Move:", flush=True)
        
        print("Poll:", self.proc.poll())
        
        string = self.proc.stdout.readline()
        
        print("read ok.")
        
        print("Read:", string, flush=True)
        
        return string.strip()
        
    

WHITE = "white"
BLACK = "black"

def parse_move(move_str: str, is_color_white: bool) -> Move:
    move_str = move_str.strip().lower()
    
    def parse_piece(p: str) -> PieceType:
        if p == 'p':
            return PieceType.PAWN
        if p == 'b':
            return PieceType.BISHOP
        if p == 'n':
            return PieceType.KNIGHT
        if p == 'r':
            return PieceType.ROOK
        if p == 'q':
            return PieceType.QUEEN
        
    if len(move_str) != 4:
        
        
        assert len(move_str) == 5, f"Length different than 5: {move_str}"
        
        is_promotion = True
        promotion_piece = parse_piece(move_str[4])
        
    else:
        is_promotion = False
        promotion_piece = PieceType.NULL
        
    def parse_col(col: str) -> int:
        return ord(col) - ord('a')
    
    def parse_row(row: str) -> int:
        return 8 - int(row)
    
    
    prev_col = parse_col(move_str[0])
    prev_row = parse_row(move_str[1])
    
    new_col = parse_col(move_str[2])
    new_row = parse_row(move_str[3])
        
    return Move(
        prev_row=prev_row,
        prev_col=prev_col,
        new_row=new_row,
        new_col=new_col,
        is_promotion=is_promotion,
        promotion_piece=promotion_piece,
        is_color_white=is_color_white
    )

def move_to_str(move: Move) -> str:
    
    def parse_col(col: int) -> str:
            cols = [
                'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'
            ]
            return cols[col]
        
    def parse_row(row: int) -> str:
        return str(8 - row)

    string = f"{parse_col(move.prev_col)}{parse_row(move.prev_row)}{parse_col(move.new_col)}{parse_row(move.new_row)}"
    
    if move.is_promotion:
        if move.promotion_piece == PieceType.BISHOP:
            string += 'b'
        if move.promotion_piece == PieceType.KNIGHT:
            string += 'n'
        if move.promotion_piece == PieceType.ROOK:
            string += 'r'
        if move.promotion_piece == PieceType.QUEEN:
            string += 'q'
        
    return string

class UniversalChessInterface:

    def __init__(
        self, 
        chessboard: TkChessboard, 
        player_white: PlayerInterface, 
        player_black: PlayerInterface
    ) -> None:
        
        self.chess_board = chessboard
        self.player_white = player_white
        self.player_black = player_black

    def run(self) -> None:
        self.player_white.start(WHITE)
        self.player_black.start(BLACK)
        
        turn = WHITE
        
        while True:
            if turn == WHITE:
                try:
                    print("[WHITE EMIT MOVE]")
                    move = self.player_white.emit_move()
                    print("[WHITE EMIT MOVE/]")
                    print()
                    
                    if move is None:
                        print("Game Over")
                        return
                    
                    print("[Loop] Move:", move, flush=True)
                    move = parse_move(move, is_color_white=True)
                    
                    print(move)
                    print()
                    
                    chessboard.make_move(move)
                    
                    print("[BLACK RECEIVE MOVE]")
                    move = move_to_str(move)
                    self.player_black.receive_move(move)
                    print("[BLACK RECEIVE MOVE/]")
                    
                    turn = BLACK
                except AssertionError as e:
                    rust: RustProcessInterface = self.player_white
                    
                    print("[WHITE EXCEPTION]")
                    
                    print("stdout:\n", rust.proc.stderr.read())
                    print("stderr:\n", rust.proc.stderr.read())
                    
                    sys.exit(1)
            
            else:
                print("[BLACK EMIT MOVE]")
                move = self.player_black.emit_move()
                print("[BLACK EMIT MOVE/]")
                print()
                
                if move is None:
                    print("Game Over", flush=True)
                    return
                
                print("[Loop] Move:", move, flush=True)
                
                move = parse_move(move, is_color_white=False)
                
                print(move)
                print()
                
                chessboard.make_move(move)
                
                print("[WHITE RECEIVE MOVE]")
                move = move_to_str(move)
                self.player_white.receive_move(move)
                print("[WHITE RECEIVE MOVE/]")
                print()
                
                turn = WHITE
            
            time.sleep(1)

if __name__ == "__main__":
    import threading
    
    root = customtkinter.CTk()
    root.geometry(Settings.get_geometry_string())
    chessboard = TkChessboard(root)

    def run():
        time.sleep(1)
        
        print("Starting Interface")
        
        chess_interface = UniversalChessInterface(
            chessboard,
            player_white=RustProcessInterface(PATH_EXECUTABLE),
            player_black=RustProcessInterface(PATH_EXECUTABLE)
        )
        
        chess_interface.run()

    engine_thread = threading.Thread(target=run)
    engine_thread.start()

    root.mainloop()