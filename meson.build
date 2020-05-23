project(
  'pathtrace',
  'cpp',
  default_options: ['cpp_std=c++17'])

threads = dependency('threads')
glew = dependency('glew')
glfw = dependency('glfw3')
glm = dependency('glm')
freeimage = meson.get_compiler('cpp').find_library('freeimage')

trace = static_library(
    'trace',
    'geometry/bounding.cc',
    'geometry/tribox.cc',
    'geometry/triray.cc',
    'kdtree/kdtree.cc',
    'trace/material.cc',
    'trace/pathtracer.cc',
    'trace/raytracer.cc',
    'trace/samplebuffer.cc',
    'trace/scene.cc',
    'trace/texture.cc',
    'util/nicetime.cc',
    'util/path.cc',
    'wavefront/mtl.cc',
    'wavefront/obj.cc',
    link_args: '-lstdc++fs',
    dependencies: [threads, glm, freeimage]
)

executable(
    'raytracer',
    'kdtree/sah.cc',
    'raytracer/main.cc',
    link_args: '-lstdc++fs',
    link_with: trace,
    dependencies: [threads, glm]
)

executable(
    'pathtracer',
    'kdtree/sah.cc',
    'pathtracer/thread.cc',
    'pathtracer/main.cc',
    link_args: '-lstdc++fs',
    link_with: trace,
    dependencies: [threads, glm]
)

executable(
    'pathtracer-gl',
    'kdtree/sah.cc',
    'pathtracer-gl/main.cc',
    'pathtracer-gl/shaders.cc',
    link_args: '-lstdc++fs',
    link_with: trace,
    dependencies: [threads, glm, glew, glfw, freeimage]
)

executable(
    'wavefront-info',
    'wavefront/wavefront-info.cc',
    link_args: '-lstdc++fs',
    link_with: trace,
    dependencies: glm
)

executable(
    'wavefront-print',
    'wavefront/wavefront-print.cc',
    link_args: '-lstdc++fs',
    link_with: trace,
    dependencies: glm
)

kdtree_builders = ['naive', 'sah']

foreach builder : kdtree_builders
  executable(
      'print-tree-' + builder,
      'kdtree/' + builder + '.cc',
      'tests/print-tree.cc',
      link_args: '-lstdc++fs',
      link_with: trace,
      dependencies: glm
  )
  executable(
      'print-tree-svg-' + builder,
      'kdtree/' + builder + '.cc',
      'tests/print-tree-svg.cc',
      link_args: '-lstdc++fs',
      link_with: trace,
      dependencies: glm
  )
endforeach

executable(
    'triangle-kdtree-tester',
    'kdtree/sah.cc',
    'tests/triangle-kdtree-tester.cc',
    link_with: trace,
    dependencies: glm
)

tests = executable(
    'unit-tests',
    'geometry/aabb_unittest.cc',
    'geometry/bounding_unittest.cc',
    'geometry/triangle_unittest.cc',
    'geometry/tribox_unittest.cc',
    'geometry/triray_unittest.cc',
    'kdtree/intersect_unittest.cc',
    'kdtree/sah_cost_unittest.cc',
    'tests/unit-tests.cc',
    'trace/camera_unittest.cc',
    'trace/fastrand_unittest.cc',
    'util/nicetime_unittest.cc',
    'wavefront/parser_unittest.cc',
    link_with: trace,
    dependencies: glm
)

test('unit tests', tests)
