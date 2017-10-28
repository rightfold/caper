#!/usr/bin/env python3

import os
import subprocess

from glob import iglob

def build_blend(blend_file, obj_path):
    command = [
        'blender',
        blend_file,
        '--background',
        '--python', 'tools/build_assets/build_blend.py',
        '--', obj_path,
    ]
    subprocess.run(command, check=True)

def main():
    target_dir = os.environ['OUT_DIR']
    for blend_file in iglob('src/**/*.blend', recursive=True):
        obj_path = target_dir + '/' + blend_file[4:-6] + '.obj'
        os.makedirs(os.path.dirname(obj_path), exist_ok=True)
        build_blend(blend_file, obj_path)

if __name__ == '__main__':
    main()
