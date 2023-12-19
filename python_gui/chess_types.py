from dataclasses import dataclass, asdict
from enum import Enum, auto

import os, os.path, pathlib

import PIL.Image
import PIL.ImageTk

FILE_PATH = os.path.dirname(__file__)
PIECES_FOLDER : pathlib.Path = pathlib.Path(FILE_PATH) / 'pieces'


class PieceType(Enum):
    NULL = 0
    PAWN = 1
    KNIGHT = 2
    BISHOP = 3
    ROOK = 4
    QUEEN = 5
    KING = 6

    @staticmethod
    def piece_name(piece) -> str:
        piece = PieceType(piece)

        if piece == PieceType.NULL:
            return "null"
        if piece == PieceType.PAWN:
            return "pawn"
        if piece == PieceType.KNIGHT:
            return "knight"
        if piece == PieceType.BISHOP:
            return "bishop"
        if piece == PieceType.ROOK:
            return "rook"
        if piece == PieceType.QUEEN:
            return "queen"
        if piece == PieceType.KING:
            return "king"

@dataclass(frozen=True)    
class Piece:
    piece_type : PieceType
    is_color_white : bool 

    def piece_color_name(self) -> str:
        return "white" if self.is_color_white else "black"

    def piece_name(self) -> str:
        return PieceType.piece_name(self.piece_type)

    def piece_path(self) -> pathlib.Path:
        filename = f"{self.piece_color_name()}_{self.piece_name()}.png"
        return PIECES_FOLDER / filename

    def piece_image(self, width : int, height : int) -> PIL.ImageTk.PhotoImage:
        im = PIL.Image.open(self.piece_path()).resize((width, height))
        return PIL.ImageTk.PhotoImage(im)


@dataclass(frozen=True, kw_only=True)
class Move:
    prev_row : int
    prev_col : int
    new_row : int
    new_col : int
    is_color_white : bool
    is_en_passant : bool = False
    is_castle : bool = False
    is_promotion : bool = False
    game_status : int = 0
    promotion_piece : PieceType = PieceType.NULL

    def to_dict(self) -> dict:
       return {key : value for key, value in asdict(self).items()}
    
def json_to_move(move_json : dict) -> Move:
    return Move(**move_json)
    


