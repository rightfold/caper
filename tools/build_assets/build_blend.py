import bpy
import sys

def export_vertices(path):
    bpy.ops.export_scene.obj(filepath=path, axis_up='Z', axis_forward='Y',
                             use_triangles=True)

def export_meta(path):
    with open(path, 'a') as file:
        for object in bpy.context.scene.objects:
            transform = ' '.join(str(component)
                                 for vector in object.matrix_world
                                 for component in vector)
            command = 'meta {} {}'.format(transform, object.name)
            print(command, file=file)

def main():
    argv = sys.argv
    argv = argv[argv.index('--') + 1:]
    if len(argv) != 1:
        print('Expecting exactly one argument.', file=sys.stderr)
        sys.exit(1)
    obj_path = argv[0]

    export_vertices(obj_path)
    export_meta(obj_path)

if __name__ == '__main__':
    main()
