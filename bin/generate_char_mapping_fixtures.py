#!/usr/bin/env python3

import os
from fontTools.ttLib import TTFont

font_base_path = 'tests/fixtures'
font_files = [
    'OpenSans-Italic.ttf',
    'SourceSerifPro-Regular.otf',
    'VeraMono-Roman.ttf',
    'MPlus2P-Regular.ttf',
]

for font_file in font_files:
    font = TTFont(f"{font_base_path}/{font_file}")
    cmap = font.get('cmap')
    for i, table in enumerate(cmap.tables):
        dir = f"tests/fixtures/char_mapping/{font_file}"
        if not os.path.exists(dir):
            os.makedirs(dir)
        f = open(f"{dir}/table{i}_format{table.format}.txt", 'w', newline='\n')
        f.write(f"{table.format}\n")
        for key, value in table.cmap.items():
            f.write(f"{str(key)} => {str(font.getGlyphID(value))}\n")
