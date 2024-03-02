use nalgebra::vector;
use pathtracer::geometry::aabb::*;
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

        let left_aabb = Aabb { center: vector![-0.36130002, -0.76769996, 0.0], half_size: vector![0.6387, 0.23349997, 1.0] };
        let right_aabb = Aabb { center: vector![-0.36130002, 0.23289996, 0.0], half_size: vector![0.6387, 0.7671, 1.0] };
        let left_triangles = [Triangle { v0: vector![-1.0, -1.0, 1.0], v1: vector![-1.0, -1.0, -1.0], v2: vector![-1.0, 1.0, 1.0] }, Triangle { v0: vector![-1.0, 1.0, -1.0], v1: vector![-1.0, 1.0, 1.0], v2: vector![-1.0, -1.0, -1.0] }, Triangle { v0: vector![-1.0, -1.0, -1.0], v1: vector![-1.0, -1.0, 1.0], v2: vector![1.0, -1.0, -1.0] }, Triangle { v0: vector![1.0, -1.0, 1.0], v1: vector![1.0, -1.0, -1.0], v2: vector![-1.0, -1.0, 1.0] }, Triangle { v0: vector![-1.0, -1.0, -1.0], v1: vector![1.0, -1.0, -1.0], v2: vector![-1.0, 1.0, -1.0] }, Triangle { v0: vector![1.0, 1.0, -1.0], v1: vector![-1.0, 1.0, -1.0], v2: vector![1.0, -1.0, -1.0] }, Triangle { v0: vector![-0.4889, -1.0005, -0.0048], v1: vector![-0.771, -1.0005, -0.4176], v2: vector![-0.3581, -1.0005, -0.6997] }, Triangle { v0: vector![-0.3581, -1.0005, -0.6997], v1: vector![-0.076, -1.0005, -0.2868], v2: vector![-0.4889, -1.0005, -0.0048] }, Triangle { v0: vector![-0.4889, -1.0005, -0.0048], v1: vector![-0.076, -1.0005, -0.2868], v2: vector![-0.076, 0.0158, -0.2868] }, Triangle { v0: vector![-0.076, 0.0158, -0.2868], v1: vector![-0.4889, 0.0158, -0.0048], v2: vector![-0.4889, -1.0005, -0.0048] }, Triangle { v0: vector![-0.076, -1.0005, -0.2868], v1: vector![-0.3581, -1.0005, -0.6997], v2: vector![-0.3581, 0.0158, -0.6997] }, Triangle { v0: vector![-0.3581, 0.0158, -0.6997], v1: vector![-0.076, 0.0158, -0.2868], v2: vector![-0.076, -1.0005, -0.2868] }, Triangle { v0: vector![-0.3581, -1.0005, -0.6997], v1: vector![-0.771, -1.0005, -0.4176], v2: vector![-0.771, 0.0158, -0.4176] }, Triangle { v0: vector![-0.771, 0.0158, -0.4176], v1: vector![-0.3581, 0.0158, -0.6997], v2: vector![-0.3581, -1.0005, -0.6997] }, Triangle { v0: vector![-0.771, -1.0005, -0.4176], v1: vector![-0.4889, -1.0005, -0.0048], v2: vector![-0.4889, 0.0158, -0.0048] }, Triangle { v0: vector![-0.4889, 0.0158, -0.0048], v1: vector![-0.771, 0.0158, -0.4176], v2: vector![-0.771, -1.0005, -0.4176] }, Triangle { v0: vector![0.2774, -1.0012, -0.0028], v1: vector![0.2774, -1.0012, 0.464], v2: vector![0.2774, -0.5342, 0.464] }, Triangle { v0: vector![0.2774, -0.5342, 0.464], v1: vector![0.2774, -0.5342, -0.0028], v2: vector![0.2774, -1.0012, -0.0028] }];
        let right_triangles = [Triangle { v0: vector![-1.0, -1.0, 1.0], v1: vector![-1.0, -1.0, -1.0], v2: vector![-1.0, 1.0, 1.0] }, Triangle { v0: vector![-1.0, 1.0, -1.0], v1: vector![-1.0, 1.0, 1.0], v2: vector![-1.0, -1.0, -1.0] }, Triangle { v0: vector![1.0, 1.0, 1.0], v1: vector![-1.0, 1.0, 1.0], v2: vector![1.0, 1.0, -1.0] }, Triangle { v0: vector![-1.0, 1.0, -1.0], v1: vector![1.0, 1.0, -1.0], v2: vector![-1.0, 1.0, 1.0] }, Triangle { v0: vector![-1.0, -1.0, -1.0], v1: vector![1.0, -1.0, -1.0], v2: vector![-1.0, 1.0, -1.0] }, Triangle { v0: vector![1.0, 1.0, -1.0], v1: vector![-1.0, 1.0, -1.0], v2: vector![1.0, -1.0, -1.0] }, Triangle { v0: vector![-0.4889, 0.0158, -0.0048], v1: vector![-0.076, 0.0158, -0.2868], v2: vector![-0.3581, 0.0158, -0.6997] }, Triangle { v0: vector![-0.3581, 0.0158, -0.6997], v1: vector![-0.771, 0.0158, -0.4176], v2: vector![-0.4889, 0.0158, -0.0048] }, Triangle { v0: vector![-0.4889, -1.0005, -0.0048], v1: vector![-0.076, -1.0005, -0.2868], v2: vector![-0.076, 0.0158, -0.2868] }, Triangle { v0: vector![-0.076, 0.0158, -0.2868], v1: vector![-0.4889, 0.0158, -0.0048], v2: vector![-0.4889, -1.0005, -0.0048] }, Triangle { v0: vector![-0.076, -1.0005, -0.2868], v1: vector![-0.3581, -1.0005, -0.6997], v2: vector![-0.3581, 0.0158, -0.6997] }, Triangle { v0: vector![-0.3581, 0.0158, -0.6997], v1: vector![-0.076, 0.0158, -0.2868], v2: vector![-0.076, -1.0005, -0.2868] }, Triangle { v0: vector![-0.3581, -1.0005, -0.6997], v1: vector![-0.771, -1.0005, -0.4176], v2: vector![-0.771, 0.0158, -0.4176] }, Triangle { v0: vector![-0.771, 0.0158, -0.4176], v1: vector![-0.3581, 0.0158, -0.6997], v2: vector![-0.3581, -1.0005, -0.6997] }, Triangle { v0: vector![-0.771, -1.0005, -0.4176], v1: vector![-0.4889, -1.0005, -0.0048], v2: vector![-0.4889, 0.0158, -0.0048] }, Triangle { v0: vector![-0.4889, 0.0158, -0.0048], v1: vector![-0.771, 0.0158, -0.4176], v2: vector![-0.771, -1.0005, -0.4176] }];

        rec.log_timeless(
            "left_aabb",
            &rerun::Boxes3D::from_centers_and_half_sizes(
                [(left_aabb.center.x, left_aabb.center.y, left_aabb.center.z)],
                [(left_aabb.half_size.x, left_aabb.half_size.y, left_aabb.half_size.z)],
            ),
        )?;
        rec.log_timeless(
            "right_aabb",
            &rerun::Boxes3D::from_centers_and_half_sizes(
                [(right_aabb.center.x, right_aabb.center.y, right_aabb.center.z)],
                [(right_aabb.half_size.x, right_aabb.half_size.y, right_aabb.half_size.z)],
            ),
        )?;

        let mut i = 0;
        for t in left_triangles {
            let points = [
                [t.v0.x, t.v0.y, t.v0.z],
                [t.v1.x, t.v1.y, t.v1.z],
                [t.v2.x, t.v2.y, t.v2.z],
                [t.v0.x, t.v0.y, t.v0.z],
            ];
            let intersecting = intersect_triangle_aabb(&t, &left_aabb);
            let good = [0, 255, 0];
            let bad = [255, 0, 0];
            let color = if intersecting { good } else { bad };
            rec.log_timeless(
                format!("left_triangle{}", i),
                &rerun::LineStrips3D::new([points]).with_colors([color]))?;
            i += 1;
        }

        let mut i = 0;
        for t in right_triangles {
            let points = [
                [t.v0.x, t.v0.y, t.v0.z],
                [t.v1.x, t.v1.y, t.v1.z],
                [t.v2.x, t.v2.y, t.v2.z],
                [t.v0.x, t.v0.y, t.v0.z],
            ];
            let intersecting = intersect_triangle_aabb(&t, &right_aabb);
            let good = [0, 0, 255];
            let bad = [255, 255, 0];
            let color = if intersecting { good } else { bad };
            rec.log_timeless(
                format!("right_triangle{}", i),
                &rerun::LineStrips3D::new([points]).with_colors([color]))?;
            i += 1;
        }

        // let mut i = 0;
        // for chunk in &obj.chunks {
        //     for face in &chunk.faces {
        //         let vertices: [[f32; 3]; 3] = [
        //             obj.index_vertex(&face.p0).into(),
        //             obj.index_vertex(&face.p1).into(),
        //             obj.index_vertex(&face.p2).into(),
        //         ];
        //         let normals: [[f32; 3]; 3] = [
        //             obj.index_normal(&face.p0).into(),
        //             obj.index_normal(&face.p1).into(),
        //             obj.index_normal(&face.p2).into(),
        //         ];
        //         let intersecting = intersect_triangle_aabb(&triangles[i], &aabb);
        //         let green: rerun::Material = rerun::Material::from_albedo_factor([0, 255, 0, 255]);
        //         let red: rerun::Material = rerun::Material::from_albedo_factor([255, 0, 0, 255]);
        //         let material = if intersecting { green } else { red };
        //         rec.log_timeless(
        //             format!("triangle{}", i),
        //             &rerun::Mesh3D::new(vertices)
        //                 .with_vertex_normals(normals)
        //                 .with_mesh_material(material)
        //         )?;
        //         i += 1;
        //     }
        // }
    }

    Ok(())
}
