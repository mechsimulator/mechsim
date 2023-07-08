use bevy::render::mesh;
use bevy::render::render_resource::PrimitiveTopology;
use serde::Deserialize;
use std::ffi::OsStr;
use std::path::PathBuf;
use std::{error::Error, path::Path};
use std::fmt;
use std::mem::size_of;
use std::fmt::Display;
use std::sync::Arc;

use bevy::{math::{DQuat, DVec3}, prelude::*};
use bincode::{self, deserialize};

#[derive(Debug, Deserialize)]
#[repr(u32)]
pub enum JointType {
    Rigid,
    Revolute,
    Slider,
}

#[derive(Debug, Deserialize, Copy, Clone)]
#[repr(C, packed)]
struct NativeDVec3 {
    x: f64,
    y: f64,
    z: f64,
}

#[derive(Debug, Deserialize, Copy, Clone)]
#[repr(C, packed)]
struct NativeDQuat {
    x: f64,
    y: f64,
    z: f64,
    w: f64,
}

#[derive(Debug, Deserialize)]
#[repr(C, packed)]
struct NativePose(NativeDVec3, NativeDQuat);

#[derive(Debug)]
pub struct Pose(pub DVec3, pub DQuat);

#[derive(Debug)]
pub struct Joint(pub JointType, pub Pose);

#[derive(Debug)]
pub struct Body {
    pub triangle_count: i32,
    pub verticies: Vec<f32>,
    pub indicies: Vec<i32>,
    pub normals: Vec<f32>,
    pub uvs: Vec<f32>,
}

#[derive(Debug)]
pub struct Part {
    pub name: String,
    pub pose: Pose,
    pub joint_references: Vec<u32>,
    pub rigid_group_references: Vec<u32>,
    pub bodies: Vec<Body>,
}

#[derive(Debug, Resource, Default)]
pub struct Assembly {
    pub joints: Vec<Joint>,
    pub parts: Vec<Part>,
}

impl Assembly {
    pub fn body_count(&self) -> usize {
        self.parts.iter()
        .map(|parts| parts.bodies.iter().count())
        .fold(0, |acc, count| acc + count)
    }
}

pub struct MrrDeserializer {
    input: Vec<u8>,
    position: usize,
}

#[derive(Debug)]
pub enum MrrError {
    FormatSigNotFound,
}

impl Display for MrrError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                MrrError::FormatSigNotFound => "format signature not found",
            }
        )
    }
}

impl Error for MrrError {}

impl MrrDeserializer {
    pub fn load(path: &Path) -> std::io::Result<Self> {
        Ok(Self {
            input: std::fs::read(path)?,
            position: 0,
        })
    }

    fn deserialize_value<'a, T: Deserialize<'a>>(&'a mut self) -> Result<T, Box<dyn Error>> {
        let data = &self.input[self.position..self.position + size_of::<T>()];
        self.position += size_of::<T>();
        Ok(deserialize(data)?)
    }

    fn deserialize_vec<'a, T, E>(&'a mut self) -> Result<T, Box<dyn Error>>
    where
        T: Deserialize<'a> + AsRef<[E]>,
        E: Deserialize<'a>,
    {
        let length = usize::from_le_bytes(
            self.input[self.position..self.position + size_of::<usize>()].try_into()?,
        );
        let vec_size = length * size_of::<E>() + size_of::<usize>();
        let vec = deserialize(&self.input[self.position..self.position + vec_size])?;
        self.position += vec_size;
        Ok(vec)
    }

    pub fn deserialize_assembly(&mut self) -> Result<Assembly, Box<dyn Error>> {
        let format_sig = "MRR (MechSim Robot Representation)";
        let mut assembly = Assembly {
            joints: vec![],
            parts: vec![],
        };

        if !self.input.starts_with(format_sig.as_bytes()) {
            return Err(Box::new(MrrError::FormatSigNotFound));
        }

        self.position += format_sig.len();

        let joint_list_size: usize = { self.deserialize_value()? };

        for _ in 0..joint_list_size {
            let joint_type: JointType = { self.deserialize_value()? };
            let native_joint_pose: NativePose = { self.deserialize_value()? };

            let vec = native_joint_pose.0;
            let quat = native_joint_pose.1;
            assembly.joints.push(Joint(
                joint_type,
                Pose(
                    DVec3::new(vec.x, vec.y, vec.z),
                    DQuat::from_xyzw(quat.x, quat.y, quat.z, quat.w),
                ),
            ));
        }

        let parts_list_size: usize = { self.deserialize_value()? };

        for _ in 0..parts_list_size {
            let name: Vec<u8> = { self.deserialize_vec()? };
            let native_part_pose: NativePose = { self.deserialize_value()? };

            let joint_reference_list: Vec<u32> = { self.deserialize_vec()? };
            let rigid_group_reference_list: Vec<u32> = { self.deserialize_vec()? };

            let body_list_size: usize = { self.deserialize_value()? };
            let mut bodies = vec![];

            for _ in 0..body_list_size {
                bodies.push(Body {
                    triangle_count: { self.deserialize_value()? },
                    verticies: { self.deserialize_vec()? },
                    indicies: { self.deserialize_vec()? },
                    normals: { self.deserialize_vec()? },
                    uvs: { self.deserialize_vec()? },
                });
            }

            let vec = native_part_pose.0;
            let quat = native_part_pose.1;

            assembly.parts.push(Part {
                name: String::from_utf8(name)?,
                pose: Pose(
                    DVec3::new(vec.x, vec.y, vec.z),
                    DQuat::from_xyzw(quat.x, quat.y, quat.z, quat.w),
                ),
                joint_references: joint_reference_list,
                rigid_group_references: rigid_group_reference_list,
                bodies: bodies,
            });
        }

        Ok(assembly)
    }
}

#[derive(Resource, Default)]
pub struct AssemblyMetadata {
    pub file_path: PathBuf,
}

impl AssemblyMetadata {
    pub fn get_name(&self) -> &str {
        self.file_path.file_stem().unwrap_or_else(|| OsStr::new("Unnamed")).to_str().unwrap_or_else(|| "[INVALID UTF-8]")
    }
}

#[derive(Resource, Default)]
pub struct AssemblyMeshes {
    pub meshes: Vec<Mesh>,
}

impl AssemblyMeshes {
    pub fn load_meshes(&mut self, assembly: &Assembly) {
        let mesh_count = assembly.body_count();
        self.meshes = Vec::with_capacity(mesh_count);

        for part in &assembly.parts {
            for body in &part.bodies {
                let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);

                mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, body.verticies.chunks(3).map(|v| [v[0] / 6., v[1] / 6., v[2] / 6.]).collect::<Vec<[f32; 3]>>());
                mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, body.verticies.chunks(3).map(|v| [v[0], v[1], v[2]]).collect::<Vec<[f32; 3]>>());
                mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, body.uvs.chunks(2).map(|v| [v[0], v[1]]).collect::<Vec<[f32; 2]>>());
                mesh.set_indices(Some(mesh::Indices::U32(body.indicies.iter().map(|&x| x as u32).collect())));

                self.meshes.push(mesh);
            } 
        }
    }
}

pub struct MrrPlugin;

impl Plugin for MrrPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Assembly>()
        .init_resource::<AssemblyMeshes>()
        .init_resource::<AssemblyMetadata>();
    }
}