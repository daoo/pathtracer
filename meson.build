project('pathtrace', 'cpp', default_options: ['cpp_std=c++1z'])

trace = static_library('trace',
  'geometry/bounding.cc',
  'geometry/tribox.cc',
  'geometry/triray.cc',
  'kdtree/array.cc',
  'kdtree/linked.cc',
  'kdtree/naive.cc',
  'kdtree/optimize.cc',
  'kdtree/surface_area_heuristic.cc',
  'kdtree/util.cc',
  'trace/material.cc',
  'trace/pathtracer.cc',
  'trace/samplebuffer.cc',
  'trace/scene.cc',
  'trace/texture.cc',
  'util/nicetime.cc',
  'util/path.cc',
  'wavefront/mtl.cc',
  'wavefront/obj.cc')

threads = dependency('threads')
glew = dependency('glew')
glfw = dependency('glfw3')
glm = dependency('glm')

executable('pathtracer',
  'pathtracer/thread.cc',
  'pathtracer/main.cc',
  link_args: ['-lstdc++fs', '-lfreeimage'],
  link_with: trace,
  dependencies: [threads, glm])

executable('pathtracer-gl',
  'pathtracer-gl/main.cc',
  'pathtracer-gl/shaders.cc',
  link_args: ['-lstdc++fs', '-lfreeimage'],
  link_with: trace,
  dependencies: [threads, glm, glew, glfw])

executable('print-wavefront',
  'tests/print-wavefront.cc',
  'wavefront/mtl.cc',
  'wavefront/obj.cc',
  link_args: '-lstdc++fs',
  dependencies: [glm])

executable('print-tree',
  'geometry/bounding.cc',
  'geometry/tribox.cc',
  'kdtree/optimize.cc',
  'kdtree/surface_area_heuristic.cc',
  'kdtree/util.cc',
  'tests/print-tree.cc',
  'trace/material.cc',
  'trace/scene.cc',
  'wavefront/obj.cc',
  link_args: '-lstdc++fs',
  dependencies: [glm])

executable('test-camera',
  'tests/camera-tester.cc',
  dependencies: [glm])

executable('sizes',
  'tests/sizes.cc',
  dependencies: [glm])
