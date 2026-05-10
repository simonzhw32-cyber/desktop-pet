#!/usr/bin/env python3
"""Generate 512x512 transparent PNG placeholder frames with a colored circle."""
import struct, zlib, os

def create_png(filename, r, g, b, label_text=None):
    width, height = 512, 512
    
    # Create pixel data: transparent background, colored circle
    rows = []
    cx, cy = 256, 256
    radius = 128
    
    for y in range(height):
        row = b'\x00'  # filter byte: None
        for x in range(width):
            dx = x - cx
            dy = y - cy
            if dx*dx + dy*dy <= radius*radius:
                row += struct.pack('BBBB', r, g, b, 255)
            else:
                row += struct.pack('BBBB', 0, 0, 0, 0)
        rows.append(row)
    
    raw_data = b''.join(rows)
    
    def make_chunk(chunk_type, data):
        chunk = chunk_type + data
        crc = struct.pack('>I', zlib.crc32(chunk) & 0xFFFFFFFF)
        return struct.pack('>I', len(data)) + chunk + crc
    
    png = b'\x89PNG\r\n\x1a\n'
    # IHDR
    ihdr_data = struct.pack('>IIBBBBB', width, height, 8, 6, 0, 0, 0)  # 8bit RGBA
    png += make_chunk(b'IHDR', ihdr_data)
    # IDAT
    compressed = zlib.compress(raw_data, 9)
    png += make_chunk(b'IDAT', compressed)
    # IEND
    png += make_chunk(b'IEND', b'')
    
    with open(filename, 'wb') as f:
        f.write(png)

# Colors for each state
states = {
    'idle': (255, 182, 193),       # pink
    'walk': (173, 216, 230),       # light blue
    'click': (255, 218, 185),      # peach
    'drag': (200, 200, 255),       # periwinkle
    'sleep': (200, 180, 220),      # lavender
    'wake': (255, 255, 180),       # light yellow
    'hover': (180, 255, 200),      # mint
    'happy': (255, 200, 200),      # salmon
    'sad': (150, 170, 210),        # steel blue
    'surprised': (255, 160, 122),  # light salmon
    'wave': (255, 228, 181),       # moccasin
}

base = os.path.dirname(os.path.abspath(__file__))
frames_dir = os.path.join(base, 'frames')

for state, color in states.items():
    filepath = os.path.join(frames_dir, f'{state}_001.png')
    create_png(filepath, *color)
    print(f'Created {filepath} ({os.path.getsize(filepath)} bytes)')

print('Done!')
