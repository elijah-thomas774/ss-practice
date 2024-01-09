import copy
from pathlib import Path
import random
from collections import Counter, OrderedDict, defaultdict

import yaml
import json
from io import BytesIO
from enum import IntEnum
from typing import Optional
import re
import struct
from extractmanager import ExtractManager

from paths import ROOT_PATH

import nlzss11
from sslib import AllPatcher, U8File
from sslib.msb import process_control_sequences
from sslib.utils import write_bytes_create_dirs, encodeBytes, toBytes
from sslib.fs_helpers import write_str, write_u16, write_float, write_u8
from sslib.dol import DOL
from sslib.rel import REL

from asm.patcher import apply_dol_patch, apply_rel_patch


class GamePatcher:
    def __init__(
        self,
        actual_extract_path,
        modified_extract_path,
    ):
        self.actual_extract_path = actual_extract_path
        self.modified_extract_path = modified_extract_path

    def do_all_gamepatches(self):
        self.load_base_patches()
        self.do_dol_patch()
        self.do_rel_patch()

    def load_base_patches(self):
        # assembly patches
        self.all_asm_patches = defaultdict(OrderedDict)

        # for asm, custom symbols
        with (ROOT_PATH / "asm" / "custom_symbols.txt").open("r") as f:
            self.custom_symbols = yaml.safe_load(f)
        self.main_custom_symbols = self.custom_symbols.get("main.dol", {})
        with (ROOT_PATH / "asm" / "original_symbols_jp.txt").open("r") as f:
            self.original_symbols = yaml.safe_load(f)
        self.main_original_symbols = self.original_symbols.get("main.dol", {})

        # for asm, free space start offset
        with (ROOT_PATH / "asm" / "free_space_start_offsets.txt").open("r") as f:
            self.free_space_start_offsets = yaml.safe_load(f)
        self.add_asm_patch("ss_necessary")

    def add_asm_patch(self, name):
        with (ROOT_PATH / "asm" / "patch_diffs" / f"{name}_diff.txt").open("r") as f:
            asm_patch_file_data = yaml.safe_load(f)
        for exec_file, patches in asm_patch_file_data.items():
            self.all_asm_patches[exec_file].update(patches)

    def do_dol_patch(self):
        # patch main.dol
        print("Patching main.dol...")
        dol_bytes = BytesIO(
            (self.actual_extract_path / "DATA" / "sys" / "main.dol").read_bytes()
        )
        dol = DOL()
        dol.read(dol_bytes)
        apply_dol_patch(self, dol, self.all_asm_patches["main.dol"])

        dol.save_changes()
        write_bytes_create_dirs(
            self.modified_extract_path / "DATA" / "sys" / "main.dol",
            dol_bytes.getbuffer(),
        )

    def do_rel_patch(self):
        rel_arc = U8File.parse_u8(
            BytesIO(
                (self.actual_extract_path / "DATA" / "files" / "rels.arc").read_bytes()
            )
        )
        rel_modified = False
        for file, codepatches in self.all_asm_patches.items():
            if file == "main.dol":  # main.dol
                continue
            rel_data = BytesIO(rel_arc.get_file_data(f"rels/{file}"))
            if rel_data is None:
                print(f"ERROR: rel {file} not found!")
                continue
            rel = REL()
            rel.read(rel_data)
            apply_rel_patch(self, rel, file, codepatches)
            rel.save_changes()
            rel_arc.set_file_data(f"rels/{file}", rel_data.getbuffer())
            rel_modified = True
        if rel_modified:
            print("Patching rels...")
            rel_data = rel_arc.to_buffer()
            write_bytes_create_dirs(
                self.modified_extract_path / "DATA" / "files" / "rels.arc",
                rel_data,
            )


if __name__ == "__main__":
    extract = ExtractManager(ROOT_PATH)
    # extract.extract_game("D:\\Dolphin_Games\\games\\SOUJ01.iso")
    patcher = GamePatcher(ROOT_PATH / "actual-extract-jp", ROOT_PATH / "modified-extract-jp")
    patcher.do_all_gamepatches()
    print("Done")
