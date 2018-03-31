project('pathtrace', 'cpp')

compiler = meson.get_compiler('cpp')
if compiler.has_argument('-std=c++17')
  add_global_arguments('-std=c++17', language: 'cpp')
elif compiler.has_argument('-std=c++1z')
  add_global_arguments('-std=c++1z', language: 'cpp')
else
  error('compiler does not support c++17')
endif

trace_source = [
    'geometry/bounding.cc',
    'geometry/tribox.cc',
    'geometry/triplane.cc',
    'geometry/triray.cc',
    'kdtree/array.cc',
    'kdtree/linked.cc',
    'kdtree/optimize.cc',
    'kdtree/surface_area_heuristic.cc',
    'trace/material.cc',
    'trace/pathtracer.cc',
    'trace/samplebuffer.cc',
    'trace/scene.cc',
    'trace/texture.cc',
    'util/nicetime.cc',
    'util/path.cc',
    'wavefront/mtl.cc',
    'wavefront/obj.cc'
]

threads = dependency('threads')
glew = dependency('glew')
glfw = dependency('glfw3')
glm = dependency('glm')
freeimage = meson.get_compiler('cpp').find_library('freeimage')

executable(
    'pathtracer',
    'pathtracer/thread.cc',
    'pathtracer/main.cc',
    trace_source,
    link_args: ['-lstdc++fs'],
    dependencies: [threads, glm, freeimage]
)

executable(
    'pathtracer-gl',
    'pathtracer-gl/main.cc',
    'pathtracer-gl/shaders.cc',
    trace_source,
    link_args: ['-lstdc++fs'],
    dependencies: [threads, glm, glew, glfw, freeimage]
)

executable(
    'print-wavefront',
    'tests/print-wavefront.cc',
    'util/nicetime.cc',
    'wavefront/mtl.cc',
    'wavefront/obj.cc',
    link_args: '-lstdc++fs',
    dependencies: [glm]
)

tree_source = [
    'geometry/bounding.cc',
    'geometry/tribox.cc',
    'geometry/triplane.cc',
    'kdtree/optimize.cc',
    'tests/print-tree.cc',
    'trace/material.cc',
    'trace/scene.cc',
    'util/nicetime.cc',
    'wavefront/obj.cc'
]

executable(
    'print-tree-naive',
    tree_source,
    'kdtree/naive.cc',
    link_args: '-lstdc++fs',
    dependencies: [glm]
)

executable(
    'print-tree-sah',
    tree_source,
    'kdtree/surface_area_heuristic.cc',
    link_args: '-lstdc++fs',
    dependencies: [glm]
)

executable(
    'print-tree-sah-fast',
    tree_source,
    'kdtree/surface_area_heuristic_fast.cc',
    link_args: '-lstdc++fs',
    dependencies: [glm]
)

executable(
    'test-camera',
    'tests/camera-tester.cc',
    dependencies: [glm]
)

executable(
    'sizes',
    'tests/sizes.cc',
    dependencies: [glm]
)

tests = executable(
    'unit-tests',
    'geometry/aabb_unittest.cc',
    'geometry/triangle_unittest.cc',
    'geometry/tribox.cc',
    'geometry/tribox_unittest.cc',
    'geometry/triplane.cc',
    'geometry/triray.cc',
    'geometry/triray_unittest.cc',
    'tests/unit-tests.cc',
    'wavefront/parser_unittest.cc',
    dependencies: [glm]
)

test('unit tests', tests)
