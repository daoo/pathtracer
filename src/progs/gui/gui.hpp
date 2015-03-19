#ifndef GUI_HPP_MKHN08BI
#define GUI_HPP_MKHN08BI

#include "trace/fastrand.hpp"
#include "trace/pathtracer.hpp"
#include "trace/samplebuffer.hpp"
#include "trace/scene.hpp"

#include <GL/glew.h>
#include <boost/filesystem/path.hpp>
#include <string>

class GUI
{
  public:
    GUI(
        const boost::filesystem::path&,
        const boost::filesystem::path&,
        const trace::Scene& scene,
        unsigned int);

    unsigned int samples() const;
    unsigned int subsampling() const;

    void increase_subsampling();
    void decrease_subsampling();

    void init_gl();
    void render() const;
    void resize(unsigned int, unsigned int);
    void trace();

    void save_screenshot() const;

  private:
    trace::FastRand m_rand;

    boost::filesystem::path m_screenshot_dir;
    boost::filesystem::path m_obj_file;

    const trace::Scene& m_scene;
    unsigned int m_camera;

    trace::Pinhole m_pinhole;
    trace::SampleBuffer m_buffer;

    unsigned int m_width, m_height;
    unsigned int m_subsampling;

    GLuint m_framebuffer_texture;
    GLuint m_shader_program;
    GLuint m_uniform_framebuffer, m_uniform_framebuffer_samples;
    GLuint m_vertex_array_object;

    void restart();
};

#endif /* end of include guard: GUI_HPP_MKHN08BI */