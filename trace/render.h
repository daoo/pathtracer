#ifndef TRACE_RENDER_H_
#define TRACE_RENDER_H_
class SampleBuffer;

namespace trace {

void Pathtracer::Render(const Scene& scene,
                        const Pinhole& pinhole,
                        Pathtracer* pathtracer,
                        SampleBuffer* buffer) {
  for (unsigned int y = 0; y < buffer->height(); ++y) {
    for (unsigned int x = 0; x < buffer->width(); ++x) {
      float sx = (static_cast<float>(x) + rand_.unit()) / buffer->width();
      float sy = (static_cast<float>(y) + rand_.unit()) / buffer->height();

      buffer->add(x, y, pathtracer->Trace(scene, pinhole.ray(sx, sy)));
    }
  }

  buffer->inc();
}
}  // namespace trace

#endif  // TRACE_RENDER_H_
