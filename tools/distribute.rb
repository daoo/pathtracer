#!/usr/bin/env ruby

require 'tmpdir'

class Render
  attr_accessor :model, :width, :height

  def initialize(model, width, height)
    raise "model does not exist #{model}" unless model and File.exists?(model)
    raise "incorrect width #{width}" unless width.is_a? Integer and width > 0
    raise "incorrect height #{height}" unless height.is_a? Integer and height > 0

    @model  = model
    @width  = width
    @height = height
  end

  def to_args()
    return "-m #{@model} -x #{@width} -y #{@height}"
  end
end

class Server
  attr_accessor :name, :dir, :threads, :samples

  def initialize(name, dir, threads, samples)
    raise "incorrect name #{name}" unless not name.empty?
    raise "incorrect dir #{dir}" unless not dir.empty?
    raise "incorrect threads #{threads}" unless threads.is_a? Integer and threads > 0
    raise "incorrect samples #{samples}" unless samples.is_a? Integer and samples > 0

    @name    = name
    @dir     = dir
    @threads = threads
    @samples = samples
  end

  def to_args()
    return "-o #{@dir} -t #{@threads} -s #{@samples}"
  end
end

def make_filename(render, samples)
  return "#{File.basename(render.model, ".obj")}_#{render.width}x#{render.height}_#{samples}.png"
end

def combine(png1, png2, output)
  IO.popen(["/usr/bin/convert", png1, png2, "-evaluate-sequence mean", output])
end

def make_archive(dir, revision)
  raise "does not exist #{dir}" unless File.directory?(dir)

  hash    = %x(git rev-parse #{revision}).chomp
  archive = "#{dir}/pathtracer-#{hash}.tar.gz"

  if not File.exists?(archive)
    system("git archive -o '#{archive}' #{revision}")
  end

  return archive
end

def transfer_archive(archive, server, dir)
end

def worker(archive, server, render, output)
  system("cat '#{archive}' | ssh #{server.name} tar -C #{server.dir} xzf -'") or raise
  system("tar xcf - #{render.model} | ssh #{server.name} tar -C #{server.dir} xzf -'") or raise
  system("ssh #{server.name} bash -s #{server.to_args()} #{render.to_args()} < tools/compile-run.sh'") or raise
  system("ssh #{server.name} 'cat TODO' > #{output}") or raise
end

def master(servers, render)
  Dir.mktmpdir("pathtracer") do |dir|
    archive = make_archive(dir, "HEAD")

    threads = []
    outputs = []
    samples = 0
    servers.each do |server|
      samples = samples + server.samples
      threads << Thread.new do
        output = "#{dir}/#{server.name}_#{server.samples}.png"
        outputs << output
        worker(archive, server, render, output)
      end
    end

    threads.each { |t| t.join }

    result_file = "build/" + make_filename(render, samples)
    outputs.each do |file|
      combine(result_file, result_file, file)
    end
  end
end

if ARGV.length != 3
  puts "Usage: distribute.rb MODEL WIDTH HEIGHT"
  puts "Servers are supplied one per line to the standard input with the following format: \"NAME,DIRECTORY,THREADS,SAMPLES\""
  exit 1
end

render = Render.new(ARGV[0], ARGV[1].to_i, ARGV[2].to_i)

servers = Array.new
STDIN.readlines.each do |line|
  a = line.split(",")
  servers << Server.new(a[0], a[1], a[2].to_i, a[3].to_i)
end

master(servers, render)
