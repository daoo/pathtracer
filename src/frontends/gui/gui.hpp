#ifndef GUI_HPP_MKHN08BI
#define GUI_HPP_MKHN08BI

#include <GL/glew.h>

#include "pathtracer/fastrand.hpp"
#include "pathtracer/pathtracer.hpp"
#include "pathtracer/samplebuffer.hpp"

#include <string>

class GUI
{
  public:
    GUI(const boost::filesystem::path&, const std::string&, const Scene& scene, size_t);

    size_t samples() const;
    size_t subsampling() const;

    void increaseSubsampling();
    void decreaseSubsampling();

    void initGL();
    void render() const;
    void resize(size_t, size_t);
    void trace();

    void saveScreenshot() const;

  private:
    FastRand m_rand;

    boost::filesystem::path m_screenshot_dir;

    const Scene& m_scene;
    const std::string& m_scene_name;
    size_t m_camera;

    Pathtracer* m_pathtracer;
    SampleBuffer* m_buffer;

    size_t m_width, m_height;
    size_t m_subsampling;

    GLuint m_framebufferTexture;
    GLuint m_shaderProgram;
    GLuint m_uniformFramebuffer, m_uniformFramebufferSamples;
    GLuint m_vertexArrayObject;

    void restart();
};

void initGUI(GUI&);
void render(GUI&);
void restart(GUI&, size_t, size_t);
void trace(GUI&);

#endif /* end of include guard: GUI_HPP_MKHN08BI */
