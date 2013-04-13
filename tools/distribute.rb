#!/usr/bin/env ruby

def combine(png1, png2, output)
  IO.popen(["/usr/bin/convert", png1, png2, "-evaluate-sequence mean", output])
end

def ssh(server, command)
  IO.popen(["/usr/bin/ssh", server] + command)
end

def work(repository, server, dir, threads, samples)
  IO.popen(["/usr/bin/rsync", "./tools/git-compile-run.sh", "#{server}:/tmp/git-compile-run.sh"])
  IO.popen(["/usr/bin/ssh", server, "/tmp/git-compile-run.sh"])
end

def master(repository, servers)
  
end
