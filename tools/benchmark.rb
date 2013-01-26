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

SCENES = ["scenes/cornell.obj", "scenes/cube.obj"]
SIZES  = [[64, 64], [128, 128], [256, 256], [512, 512], [1024, 1024]]

case ARGV[0]
  when "threads"
    WIDTH  = 512
    HEIGHT = 512

    print "Threads"
    SCENES.map { |s| print ",", s }
    puts

    (1..8).each do |threads|
      print threads
      SCENES.each do |scene|
        print ","
        print measure(scene, WIDTH, HEIGHT, threads, threads * 4)
      end
      puts
    end
  when "size"
    THREADS = 1
    SAMPLES = 32

    print "Sizes"
    SCENES.map { |s| print ",", s }
    puts

    SIZES.each do |size|
      print size[0], "x", size[1]
      SCENES.each do |scene|
        print ","
        print measure(scene, size[0], size[1], THREADS, SAMPLES)
      end
      puts
    end
  else
    puts "Usage: benchmark.rb [threads|size]"
    exit 1
end
