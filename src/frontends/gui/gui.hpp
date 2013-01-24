#ifndef GUI_HPP_MKHN08BI
#define GUI_HPP_MKHN08BI

#include "trace/fastrand.hpp"
#include "trace/pathtracer.hpp"
#include "trace/samplebuffer.hpp"

#include <GL/glew.h>
#include <string>

class GUI
{
  public:
    GUI(const boost::filesystem::path&, const std::string&, const trace::Scene& scene, unsigned int);

    unsigned int samples() const;
    unsigned int subsampling() const;

    void increaseSubsampling();
    void decreaseSubsampling();

    void initGL();
    void render() const;
    void resize(unsigned int, unsigned int);
    void trace();

    void saveScreenshot() const;

  private:
    trace::FastRand m_rand;

    boost::filesystem::path m_screenshot_dir;

    const trace::Scene& m_scene;
    const std::string& m_scene_name;
    unsigned int m_camera;

    trace::Pathtracer* m_pathtracer;
    trace::SampleBuffer* m_buffer;

    unsigned int m_width, m_height;
    unsigned int m_subsampling;

    GLuint m_framebufferTexture;
    GLuint m_shaderProgram;
    GLuint m_uniformFramebuffer, m_uniformFramebufferSamples;
    GLuint m_vertexArrayObject;

    void restart();
};

#endif /* end of include guard: GUI_HPP_MKHN08BI */
