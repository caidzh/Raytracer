use crate::bvh::BvhNode;
use crate::material::Material;
use crate::triangle::Triangle;
use crate::vec3::Vector;
use crate::{hittable_list::HittableList, material::Lambertian};
use std::sync::Arc;

pub fn get_obj(obj_filename: &str, scale: f64) -> HittableList {
    let mut val = HittableList::new();
    let obj = tobj::load_obj(
        format!("objects/{}", obj_filename),
        &tobj::LoadOptions {
            single_index: false,
            triangulate: false,
            ignore_points: true,
            ignore_lines: true,
        },
    );
    let (models, materials) = obj.expect("Failed to load OBJ file");
    let materials = materials.expect("Failed to load MTL file");
    for (_i, m) in models.iter().enumerate() {
        let mesh = &m.mesh;
        let mut mat: Arc<dyn Material> = Arc::new(Lambertian::new(Vector::new(0.0, 0.0, 0.0)));
        let id = mesh.material_id;
        if let Some(diffuse) = materials.clone()[id.unwrap()].diffuse {
            mat = Arc::new(Lambertian::new(Vector::new(
                diffuse[0] as f64,
                diffuse[1] as f64,
                diffuse[2] as f64,
            )));
        }
        if !mesh.face_arities.is_empty() {
            let mut next_face = 0;
            for f in 0..mesh.face_arities.len() {
                let end = next_face + mesh.face_arities[f] as usize;
                let face_indices: Vec<_> = mesh.indices[next_face..end].iter().collect();
                next_face = end;
                let mut point: [Vector; 3] = [Vector::new(0.0, 0.0, 0.0); 3];
                let mut t = 0;
                let p0 = *face_indices[0] as usize;
                point[0] = Vector::new(
                    mesh.positions[3 * p0] as f64 * scale,
                    mesh.positions[3 * p0 + 1] as f64 * scale,
                    mesh.positions[3 * p0 + 2] as f64 * scale,
                );
                for v in face_indices {
                    t += 1;
                    point[1] = point[2];
                    point[2] = Vector::new(
                        mesh.positions[3 * (*v as usize)] as f64 * scale,
                        mesh.positions[3 * (*v as usize) + 1] as f64 * scale,
                        mesh.positions[3 * (*v as usize) + 2] as f64 * scale,
                    );
                    if t >= 3 {
                        val.add(Arc::new(Triangle::new(point[0], point[1], point[2], mat.clone())));
                    }
                }
            }
        } else {
            let mut point: [Vector; 3] = [Vector::new(0.0, 0.0, 0.0); 3];
            let mut t = 0;
            for v in &mesh.indices {
                point[t] = Vector::new(
                    mesh.positions[3 * (*v as usize)] as f64 * scale,
                    mesh.positions[3 * (*v as usize) + 1] as f64 * scale,
                    mesh.positions[3 * (*v as usize) + 2] as f64 * scale,
                );
                t += 1;
                if t == 3 {
                    val.add(Arc::new(Triangle::new(point[0], point[1], point[2], mat.clone())));
                    t = 0;
                }
            }
        }
    }
    // for (i, m) in models.iter().enumerate() {
    //     let mesh = &m.mesh;

    //     println!("model[{}].name = \'{}\'", i, m.name);
    //     println!("model[{}].mesh.material_id = {:?}", i, mesh.material_id);

    //     println!(
    //         "Size of model[{}].face_arities: {}",
    //         i,
    //         mesh.face_arities.len()
    //     );

    //     let mut next_face = 0;
    //     for f in 0..mesh.face_arities.len() {
    //         let end = next_face + mesh.face_arities[f] as usize;
    //         let face_indices: Vec<_> = mesh.indices[next_face..end].iter().collect();
    //         println!("    face[{}] = {:?}", f, face_indices);
    //         next_face = end;
    //     }

    //     // Normals and texture coordinates are also loaded, but not printed in this example
    //     println!("model[{}].vertices: {}", i, mesh.positions.len() / 3);

    //     assert!(mesh.positions.len() % 3 == 0);
    //     for v in 0..mesh.positions.len() / 3 {
    //         println!(
    //             "    v[{}] = ({}, {}, {})",
    //             v,
    //             mesh.positions[3 * v],
    //             mesh.positions[3 * v + 1],
    //             mesh.positions[3 * v + 2]
    //         );
    //     }
    // }

    // for (i, m) in materials.iter().enumerate() {
    //     println!("material[{}].name = \'{}\'", i, m.name);
    //     if let Some(ambient) = m.ambient {
    //         println!(
    //             "    material.Ka = ({}, {}, {})",
    //             ambient[0], ambient[1], ambient[2]
    //         );
    //     }
    //     if let Some(diffuse) = m.diffuse {
    //         println!(
    //             "    material.Kd = ({}, {}, {})",
    //             diffuse[0], diffuse[1], diffuse[2]
    //         );
    //     }
    //     if let Some(specular) = m.specular {
    //         println!(
    //             "    material.Ks = ({}, {}, {})",
    //             specular[0], specular[1], specular[2]
    //         );
    //     }
    //     if let Some(shininess) = m.shininess {
    //         println!("    material.Ns = {}", shininess);
    //     }
    //     if let Some(dissolve) = m.dissolve {
    //         println!("    material.d = {}", dissolve);
    //     }
    //     if let Some(ambient_texture) = &m.ambient_texture {
    //         println!("    material.map_Ka = {}", ambient_texture);
    //     }
    //     if let Some(diffuse_texture) = &m.diffuse_texture {
    //         println!("    material.map_Kd = {}", diffuse_texture);
    //     }
    //     if let Some(specular_texture) = &m.specular_texture {
    //         println!("    material.map_Ks = {}", specular_texture);
    //     }
    //     if let Some(shininess_texture) = &m.shininess_texture {
    //         println!("    material.map_Ns = {}", shininess_texture);
    //     }
    //     if let Some(normal_texture) = &m.normal_texture {
    //         println!("    material.map_Bump = {}", normal_texture);
    //     }
    //     if let Some(dissolve_texture) = &m.dissolve_texture {
    //         println!("    material.map_d = {}", dissolve_texture);
    //     }

    //     for (k, v) in &m.unknown_param {
    //         println!("    material.{} = {}", k, v);
    //     }
    // }
    let mut bvh_val = HittableList::new();
    bvh_val.initialise(Arc::new(BvhNode::initialise(&mut val)));
    bvh_val
}
