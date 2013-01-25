#!/usr/bin/env ruby

require 'open3'

COMMAND = "./build/opt/src/frontends/cmd/pathtracer"

def measure(scene, width, height, threads, samples)
  Open3.popen3(COMMAND,
               '-x', width.to_s,
               '-y', height.to_s,
               '-s', samples.to_s,
               '-t', threads.to_s,
               scene) do |i, o, e|
    return o.readlines[-2].split[3].to_f
  end
end

SCENES  = ["scenes/cornell.obj", "scenes/cube.obj"]
SIZES   = [[64, 64], [128, 128], [256, 256], [512, 512], [1024, 1024]]
THREADS = [1, 2, 3, 4, 5, 6, 7, 8]

SCENES.each do |scene|
  SIZES.each do |size|
    THREADS.each do |threads|
      print scene, ",", size[0], "x", size[1], ",", threads, ","
      puts measure(scene, size[0], size[1], threads, threads * 4)
    end
  end
end
