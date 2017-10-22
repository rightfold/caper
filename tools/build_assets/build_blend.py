import bpy
import sys

def main():
    argv = sys.argv
    argv = argv[argv.index('--') + 1:]
    if len(argv) != 1:
        print('Expecting exactly one argument.', file=sys.stderr)
    obj_path = argv[0]
    bpy.ops.export_scene.obj(filepath=obj_path, axis_up='Z', use_triangles=True)

if __name__ == '__main__':
    main()
