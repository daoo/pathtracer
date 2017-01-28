#include "pathtracer/thread.hpp"

#include <boost/program_options.hpp>
#include <experimental/filesystem>
#include <iostream>
#include <sstream>
#include <string>

namespace fs = std::experimental::filesystem;
namespace po = boost::program_options;

using namespace std;

constexpr unsigned int WIDTH = 512;
constexpr unsigned int HEIGHT = 512;
constexpr unsigned int THREADS = 1;
constexpr unsigned int SAMPLES = 128;

constexpr int OK = 0;
constexpr int ERROR_PARAMS = 1;
constexpr int ERROR_FILE_NOT_FOUND = 2;
constexpr int ERROR_PROGRAM = 3;

int main(int argc, char* argv[]) {
  fs::path outdir, model;
  unsigned int width, height;
  unsigned int samples, threads;

  po::options_description desc("Pathtracer options");
  desc.add_options()("help,h", "produce help message")(
      "model,m", po::value<fs::path>(&model), "obj model")(
      "outdir,o", po::value<fs::path>(&outdir),
      "output directory for resulting image, if not specified, no image is "
      "written")("threads,t",
                 po::value<unsigned int>(&threads)->default_value(THREADS),
                 "number of threads to use")(
      "width,x", po::value<unsigned int>(&width)->default_value(WIDTH),
      "width of the image")(
      "height,y", po::value<unsigned int>(&height)->default_value(HEIGHT),
      "height of the image")(
      "samples,s", po::value<unsigned int>(&samples)->default_value(SAMPLES),
      "number of samples to render");

  try {
    po::positional_options_description pd;
    pd.add("model", -1);

    po::variables_map vm;
    po::store(
        po::command_line_parser(argc, argv).options(desc).positional(pd).run(),
        vm);
    po::notify(vm);

    if (vm.count("help")) {
      cout << desc << '\n';
      return OK;
    }
  } catch (const po::error& ex) {
    cerr << "ERROR: " << ex.what() << "\n\n";
    cout << desc;
    return ERROR_PARAMS;
  }

  if (!exists(model)) {
    cerr << "ERROR: file " << model << " does not exist.\n";
    return ERROR_FILE_NOT_FOUND;
  }

  try {
    program(model, outdir, width, height, 0, samples, threads);
  } catch (const string& str) {
    cerr << str << '\n';
    return ERROR_PROGRAM;
  }

  return OK;
}
