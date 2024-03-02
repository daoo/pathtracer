use pathtracer::geometry::bounding::*;
use pathtracer::geometry::intersect::*;
use pathtracer::geometry::triangle::*;
use pathtracer::wavefront::*;
use std::env;
use std::fs;
use std::path::Path;
use std::str;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rec = rerun::RecordingStreamBuilder::new("reruntest").spawn()?;
    for arg in env::args().skip(1) {
        let path = Path::new(&arg);
        let bytes = fs::read(path).unwrap();
        let input = str::from_utf8(&bytes).unwrap();
        let obj = obj::obj(&input);
        let mut triangles: Vec<Triangle> = Vec::new();
        for chunk in &obj.chunks {
            for face in &chunk.faces {
                triangles.push(
                    Triangle{
                        v0: obj.index_vertex(&face.p0),
                        v1: obj.index_vertex(&face.p1),
                        v2: obj.index_vertex(&face.p2),
                    }
                );
            }
        }

        let aabb = bounding(&triangles);
        rec.log_timeless(
            "bounding_box",
            &rerun::Boxes3D::from_centers_and_half_sizes(
                [(aabb.center.x, aabb.center.y, aabb.center.z)],
                [(aabb.half_size.x, aabb.half_size.y, aabb.half_size.z)],
            ),
        )?;

        let mut i = 0;
        for chunk in &obj.chunks {
            for face in &chunk.faces {
                let vertices: [[f32; 3]; 3] = [
                    obj.index_vertex(&face.p0).into(),
                    obj.index_vertex(&face.p1).into(),
                    obj.index_vertex(&face.p2).into(),
                ];
                let normals: [[f32; 3]; 3] = [
                    obj.index_normal(&face.p0).into(),
                    obj.index_normal(&face.p1).into(),
                    obj.index_normal(&face.p2).into(),
                ];
                let intersecting = intersect_triangle_aabb(&triangles[i], &aabb);
                let green: rerun::Material = rerun::Material::from_albedo_factor([0, 255, 0, 255]);
                let red: rerun::Material = rerun::Material::from_albedo_factor([255, 0, 0, 255]);
                let material = if intersecting { green } else { red };
                rec.log_timeless(
                    format!("triangle{}", i),
                    &rerun::Mesh3D::new(vertices)
                        .with_vertex_normals(normals)
                        .with_mesh_material(material)
                )?;
                i += 1;
            }
        }
    }

    Ok(())
}
