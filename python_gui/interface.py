import subprocess
import pathlib

import json

EXEC_PATH = pathlib.Path("/home/cabral/Documents/projects/complete_chess/cxx_engine/output")


def parse_bytes(byte0, byte1):
    new_row = (byte0 >> 2) & (0b111)
    new_col = (byte0 >> 5) & (0b111) 

    prev_row = (byte1) & (0b111)
    prev_col = (byte1 >> 3)  & (0b111)

    is_over = (byte1 >> 7) & 1
    game_status = (byte1 >> 1) & (0b111)

    return (is_over, game_status), ((prev_row, prev_col), (new_row, new_col))

def int_to_char(n):
    return chr(n + ord('a'))

def print_move(move):
    prev_row = 8 - move[0][0]
    prev_col = int_to_char(move[0][1])

    new_row = 8 - move[1][0]
    new_col = int_to_char(move[1][1])

    print(f"{prev_col}{prev_row} -> {new_col}{new_row}")


class Interface:
    def __init__(self):
        self.proc = subprocess.Popen(EXEC_PATH, stdout=subprocess.PIPE, stderr=subprocess.PIPE)

        self._end_status = None
        self._done = False

    def get_message(self):

        skip_char = [" ", "\t", "\n"]

        char = self.proc.stdout.read(1).decode()

        while char in skip_char:
            char = self.proc.stdout.read(1).decode()

        if char != "{":
            print("Char is", char)

        assert char == "{"

        message = ""

        while char != "}":
            message += char
            char = self.proc.stdout.read(1).decode()

        message += "}"

        message_struct = json.loads(message)

        return message_struct



    def read_update(self):
        byte0 = self.proc.stdout.read(1)[0]
        byte1 = self.proc.stdout.read(1)[0]

        status, move = parse_bytes(byte0, byte1)

        if status[0] != 0:
            self._done = True
            self._end_status = status[1]

        return move
    
    def is_done(self) -> bool:
        return self._done

    def end_status(self):
        if not self._done:
            raise Exception("Game is not over yet.")
        return self._end_status


if __name__ == "__main__":
    interface = Interface()
    while not interface.is_done():
        interface.get_message()
        # print_move(interface.read_update())

    